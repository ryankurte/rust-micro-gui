//! Graphics module provides underlying rendering functions over a generic buffer
//! This is used by components or widgets for rendering purposes.
//!
//! Copyright 2017 Ryan Kurte


use types::*;

/// Graphics context used for rendering components
/// This includes offsets and dimensions to shift rendering scopes
pub struct Graphics {
    x: usize,
    y: usize,
    w: usize,
    h: usize
}


impl Graphics {
    /// New creates a new graphcs context with the provided offsets and limits
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Graphics {
        return Graphics{x, y, w, h};
    }

    /// Set wraps a buffer in a graphics context to shift rendering functions
    pub fn set(&self, b: &mut buffer::Set, x: usize, y: usize, p: &pixel::Pixel) {
        let new_x = self.x + x;
        let new_y = self.y + y;

        if !(x > self.w || y > self.h) {
            b.set(new_x, new_y, p);
        }
    }

    /// Draws a line between two points with the provided pixel style
    pub fn draw_line(&self, buf: &mut buffer::Set, p1: point::Point, p2: point::Point, p: &pixel::Pixel) {
        // Bresenham's line algorithm (https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm), implementation from:
	    // https://www.opengl.org/discussion_boards/showthread.php/168761-Drawing-Line-Bresenhem-midpoint-algorithm

        let delta_x = (p1.x as isize - p2.x as isize).abs();
        let delta_y = (p1.y as isize - p2.y as isize).abs();
        let sign_x = (p1.x as isize - p2.x as isize).signum();
        let sign_y = (p1.y as isize - p2.y as isize).signum();

        self.set(buf, p1.x, p1.y, p);

        let mut a = p1;
        let mut b = p2;

        b.x = (b.x as isize + sign_x) as usize;
        b.y = (b.y as isize + sign_y) as usize;

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

                if a.x == b.x { break; }
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

                if a.y == b.y { break; }
            }
        }
    }

    /// Draws a rectange with the provided pixel style
    pub fn draw_rect(&self, b: &mut buffer::Set, r: rect::Rect, p: &pixel::Pixel) {
        for x in r.x..r.w {
            self.set(b, x, r.y, p);
            self.set(b, x, r.y + r.h, p);
        }
        for y in r.y..r.h {
            self.set(b, r.x, y, p);
            self.set(b, r.x + r.w, y, p);
        }
    }

    pub fn draw_ellipse(&self, buf: &mut buffer::Set, r: rect::Rect, p: &pixel::Pixel) {
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

                if y <= 0 { break; }
            }

        }
    }

    //pub fn draw_sprite(&self, b: &mut buffer::Set, p: &point::Point, s: &sprite::Sprite) { }
    //pub fn draw_text(&self, b: &mut buffer::Set, ) { }
}