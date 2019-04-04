
use std::*;

use crate::core::buffer::Buff;
use crate::types::rect::Rect;
use crate::graphics::{Graphics, Renderable};

/// Layers combine graphics functions to provide reusable blocks for rendering
pub struct Layer<'a, Pixel> {
    bounds: Rect,
    visible: bool,
    renderer: Option<&'a mut (Renderable<Pixel> + 'a)>,
    children: vec::Vec<&'a mut Layer<'a, Pixel>>
}

impl <'a, Pixel>Layer<'a, Pixel> {
    /// Create a new layer with the provided bounds
    pub fn new(bounds: Rect, renderer: Option<&'a mut (Renderable<Pixel> + 'a)>) -> Layer<'a, Pixel> {
        return Layer{bounds: bounds, visible: true, renderer, children: Vec::new()};
    }

    /// Fetch the bounds of a given layer
    pub fn bounds(&self) -> Rect {
        return self.bounds;
    }

    /// Set layer visible state
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl <'a, Pixel>Renderable<Pixel> for Layer<'a, Pixel> {
    /// Render a layer using the provided graphics context and buffer
    fn render(&mut self, graphics: &mut Graphics<Pixel>, buffer: &mut Buff<Pixel>) {

        if !self.visible {
            return;
        }

        // Update graphics context with new layer bounds
        let gfx_bounds = graphics.get_bounds();
        let mut new_bounds = gfx_bounds;
        new_bounds.x += self.bounds.x;
        new_bounds.y += self.bounds.y;
        new_bounds.w = if gfx_bounds.w > self.bounds.w { self.bounds.w } else { gfx_bounds.w };
        new_bounds.h = if gfx_bounds.h > self.bounds.h { self.bounds.h } else { gfx_bounds.h };
        graphics.set_bounds(&new_bounds);

        // Render children
        for child in self.children.iter_mut() {
            child.render(graphics, buffer);
        }

        // Render parent
        match self.renderer {
            Some(ref mut r) => r.render(graphics, buffer),
            None => ()
        }

        // Revert graphics context
        graphics.set_bounds(&gfx_bounds);

    }
}
