//! Graphics module provides underlying rendering functions over a generic buffer
//! This is used by components or widgets for rendering purposes.
//!
//! Copyright 2019 Ryan Kurte

use std::marker::PhantomData;

use crate::core::buffer::Buff;
use crate::types::point::Point;
use crate::types::rect::Rect;

/// Renderable trait implemented by types that can render themselves
/// For example, widgets should implement the renderable trait to be bound into layers
pub trait Renderable<Pixel> {
    fn render(&mut self, graphics: &mut Graphics<Pixel>, buffer: &mut Buff<Pixel>);
}

/// Sprite trait implemented by types that can be rendered from a buffer
pub trait Sprite<Pixel> {
    fn get(&mut self, x: usize, y: usize) -> &Pixel;
    fn size(&self) -> (usize, usize);
}

/// Graphics context used for rendering components
/// This includes offsets and dimensions to shift rendering scopes
pub struct Graphics<Pixel> {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    _pixel: PhantomData<Pixel>,
}

impl<Pixel> Graphics<Pixel> {
    /// New creates a new graphcs context with the provided offsets and limits
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        return Self {
            x,
            y,
            w,
            h,
            _pixel: PhantomData,
        };
    }

    /// Set wraps a buffer in a graphics context to shift rendering functions
    pub fn set(&self, b: &mut Buff<Pixel>, x: usize, y: usize, p: &Pixel) {
        let new_x = self.x + x;
        let new_y = self.y + y;

        if x < self.w && y < self.h {
            b.set(new_x, new_y, p);
        }
    }

    pub fn get_bounds(&mut self) -> Rect {
        return Rect::new(self.x, self.y, self.w, self.h);
    }

    /// Set new bounds for rendering
    /// This will offset and limit rendering by the provided values.
    pub fn set_bounds(&mut self, bounds: &Rect) {
        self.x = bounds.x;
        self.y = bounds.y;
        self.w = bounds.w;
        self.h = bounds.h;
    }

    fn limit(min: usize, max: usize, actual: usize) -> usize {
        if actual > max {
            return max;
        }
        if actual < min {
            return min;
        }
        return actual;
    }

    /// Draws a line between two points with the provided pixel style
    pub fn draw_line(&self, buf: &mut Buff<Pixel>, p1: Point, p2: Point, p: &Pixel) {
        // Bresenham's line algorithm (https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm), implementation from:
        // https://www.opengl.org/discussion_boards/showthread.php/168761-Drawing-Line-Bresenhem-midpoint-algorithm

        // Create local mutable copies of the required points
        let mut a = p1;
        let mut b = p2;

        // Limit points to drawable space
        //a.x = self::limit(0, self.w, a.x);
        //a.y = self::limit(0, self.h, a.y);
        //b.x = self::limit(0, self.w, b.x);
        //b.y = self::limit(0, self.h, b.y);

        let mut delta_x = b.x as isize - a.x as isize;
        let mut delta_y = b.y as isize - a.y as isize;

        let sign_x = delta_x.signum();
        let sign_y = delta_y.signum();

        delta_x = delta_x.abs();
        delta_y = delta_y.abs();

        b.x = (b.x as isize + sign_x) as usize;
        b.y = (b.y as isize + sign_y) as usize;

        self.set(buf, a.x, a.y, p);

        if delta_x > delta_y {
            let mut accum = delta_x / 2;
            loop {
                self.set(buf, a.x, a.y, p);
                accum -= delta_y;
                if accum < 0 {
                    accum += delta_x;
                    a.y = (a.y as isize + sign_y) as usize;
                }
                a.x = (a.x as isize + sign_x) as usize;

                if a.x == b.x {
                    break;
                }
            }
        } else {
            let mut accum = delta_y / 2;
            loop {
                self.set(buf, a.x, a.y, p);

                accum -= delta_x;
                if accum < 0 {
                    accum += delta_y;
                    a.x = (a.x as isize + sign_x) as usize;
                }
                a.y = (a.y as isize + sign_y) as usize;

                if a.y == b.y {
                    break;
                }
            }
        }
    }

    /// Draws a rectange with the provided pixel style
    pub fn draw_rect(&self, b: &mut Buff<Pixel>, r: Rect, p: &Pixel) {
        for x in 0..r.w {
            self.set(b, r.x + x, r.y, p);
            self.set(b, r.x + x, r.y + r.h, p);
        }
        for y in 0..r.h {
            self.set(b, r.x, r.y + y, p);
            self.set(b, r.x + r.w, r.y + y, p);
        }
    }

    /// Draws a rectangle with the provided pixel style
    pub fn fill_rect(&self, b: &mut Buff<Pixel>, r: Rect, p: &Pixel) {
        for y in 0..r.h {
            for x in 0..r.w {
                self.set(b, r.x + x, r.y + y, p);
            }
        }
    }

    /// Draws a polyline connecting a list of points
    pub fn draw_polyline(&self, b: &mut Buff<Pixel>, points: &[Point], p: &Pixel) {
        let len = points.len();
        for i in 0..len - 1 {
            let p1 = points[i];
            let p2 = points[i + 1];
            self.draw_line(b, p1, p2, p);
        }
    }

    /// Draws an ellipse to fill the provided rectangle
    pub fn draw_ellipse(&self, buf: &mut Buff<Pixel>, r: Rect, p: &Pixel) {
        // Implementation also from:
        // https://www.opengl.org/discussion_boards/showthread.php/168761-Drawing-Line-Bresenhem-midpoint-algorithm

        let left = (r.x) as isize;
        let right = (r.x + r.w) as isize;
        let top = (r.y) as isize;
        let bottom = (r.y + r.h) as isize;

        let a = ((right - left + 1) / 2) as isize;
        let b = ((bottom - top + 1) / 2) as isize;

        if a != 0 && b != 0 {
            let a2 = (a * a) as isize;
            let b2 = (b * b) as isize;
            let two_a2 = (a2 * 2) as isize;
            let two_b2 = (b2 * 2) as isize;
            let four_a2 = (a2 * 4) as isize;
            let four_b2 = (b2 * 4) as isize;
            let mut x = 0 as isize;
            let mut y = b as isize;
            let mut s = a2 * (1 - (b * 2)) + two_b2;
            let mut t = b2 - two_a2 * ((b * 2) - 1);

            self.set(buf, (right + x - a) as usize, (bottom + y - b) as usize, p);
            self.set(buf, (left - x + a) as usize, (bottom + y - b) as usize, p);
            self.set(buf, (left - x + a) as usize, (top - y + b) as usize, p);
            self.set(buf, (right + x - a) as usize, (top - y + b) as usize, p);

            loop {
                if s < 0 {
                    s += two_b2 * ((x << 1) + 3);
                    t += four_b2 * (x + 1);
                    x += 1;
                } else if t < 0 {
                    s += two_b2 * ((x << 1) + 3) - four_a2 * (y - 1);
                    t += four_b2 * (x + 1) - two_a2 * ((y << 1) - 3);
                    x += 1;
                    y -= 1;
                } else {
                    s -= four_a2 * (y - 1);
                    t -= two_a2 * ((y << 1) - 3);
                    y -= 1;
                }

                self.set(buf, (right + x - a) as usize, (bottom + y - b) as usize, p);
                self.set(buf, (left - x + a) as usize, (bottom + y - b) as usize, p);
                self.set(buf, (left - x + a) as usize, (top - y + b) as usize, p);
                self.set(buf, (right + x - a) as usize, (top - y + b) as usize, p);

                if y <= 0 {
                    break;
                }
            }
        }
    }

    //pub fn draw_sprite(&self, b: &mut Buff<Pixel>, p: &point::Point, s: &sprite::Sprite) { }
    //pub fn draw_text(&self, b: &mut Buff<Pixel>, ) { }
}
