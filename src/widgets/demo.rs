
use types::pixel::*;
use types::rect::Rect;
use types::point::Point;
use types::buffer::Buff;
use graphics::{Graphics, Renderable};

pub struct DemoWidget {

}

impl DemoWidget {
    pub fn new() -> DemoWidget {
        return DemoWidget{}
    }
}

impl Renderable for DemoWidget {
    fn render(&mut self, graphics: &mut Graphics, buffer: &mut Buff) {
        let bounds = graphics.get_bounds();

        // Lines
        graphics.draw_line(buffer, Point{x: 20, y: 20}, Point{x: bounds.w - 20, y: 20}, &Pixel::black());
        graphics.draw_line(buffer, Point{x: 0, y: 0}, Point{x: bounds.w, y: bounds.h}, &Pixel::black());
        graphics.draw_line(buffer, Point{x: 0, y: bounds.h}, Point{x: bounds.w, y: 0}, &Pixel::black());

        let points = [
                Point{x: bounds.w/6*1, y: bounds.h/8*2}, 
                Point{x: bounds.w/6*2, y: bounds.h/8*1},
                Point{x: bounds.w/6*3, y: bounds.h/8*2},
                Point{x: bounds.w/6*4, y: bounds.h/8*1},
                Point{x: bounds.w/6*5, y: bounds.h/8*2}];
        graphics.draw_polyline(buffer, &points, &Pixel::black());

        // Circles
        let ellipse_r = bounds.w/4;
        let circles = [
            Rect{x: (bounds.w-ellipse_r)/2 - ellipse_r/5*3, y: (bounds.h-ellipse_r)/2, w: ellipse_r, h: ellipse_r},
            Rect{x: (bounds.w-ellipse_r)/2, y: (bounds.h-ellipse_r)/2, w: ellipse_r, h: ellipse_r},
            Rect{x: (bounds.w-ellipse_r)/2 + ellipse_r/5*3, y: (bounds.h-ellipse_r)/2, w: ellipse_r, h: ellipse_r}
        ];
        graphics.draw_ellipse(buffer, circles[0], &Pixel::red());
        graphics.draw_ellipse(buffer, circles[1], &Pixel::green());
        graphics.draw_ellipse(buffer, circles[2], &Pixel::blue());

        // Rectangles
        graphics.draw_rect(buffer, Rect::new(bounds.w/7*1-16, bounds.h/8*6-16, bounds.w/7*5+32, bounds.h/6+32), &Pixel::black());
        graphics.fill_rect(buffer, Rect::new(bounds.w/7*1, bounds.h/8*6, bounds.w/7, bounds.h/6), &Pixel::red());
        graphics.fill_rect(buffer, Rect::new(bounds.w/7*3, bounds.h/8*6, bounds.w/7, bounds.h/6), &Pixel::green());
        graphics.fill_rect(buffer, Rect::new(bounds.w/7*5, bounds.h/8*6, bounds.w/7, bounds.h/6), &Pixel::blue());

    }

}
