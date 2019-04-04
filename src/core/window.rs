//! Window implements the top level GUI abstraction.
//! Windows can create and consume events, and contain a base layer from which windows can be implemented.
//!
//! Copyright 2017 Ryan Kurte

use std::*;


use types::*;

use super::buffer::Buff;
use super::layer::Layer;

use graphics::{Graphics, Renderable};

/// Window object contains renderable layers and handles events
pub struct Window<'a, Pixel> {
    base: Layer<'a, Pixel>,
    on_load: Option<&'a mut (OnLoad + 'a)>,
    on_unload: Option<&'a mut (OnUnload + 'a)>,
    on_event: Option<&'a mut (OnEvent + 'a)>
}

/// OnLoad trait called when a window is loaded
pub trait OnLoad {
    fn on_load(&mut self);
}

/// OnUnload trait called when a window is unloaded
pub trait OnUnload {
    fn on_unload(&mut self);
}

/// OnEvent trait called to receive an event
pub trait OnEvent {
    fn on_event(&mut self, e: &Event);
}

impl <'a, Pixel>Window<'a, Pixel> {
    /// Create a new window for rendering
    pub fn new(w: usize, h: usize, renderer: Option<&'a mut (Renderable<Pixel> + 'a)>) -> Self {
        let bounds = Rect::new(0, 0, w, h);
        let base = Layer::new(bounds, renderer);
        return Window{base: base, on_load: None, on_event: None, on_unload: None}
    }

    /// Bind optional load, unload, and event handlers.
    pub fn bind_handlers(&mut self, on_load: Option<&'a mut OnLoad>, on_unload: Option<&'a mut OnUnload>, on_event: Option<&'a mut OnEvent>) {
        self.on_load = on_load;
        self.on_unload = on_unload;
        self.on_event = on_event;
    }

    /// Render the layer
    pub fn render(&mut self, graphics: &mut Graphics<Pixel>, buffer: &mut Buff<Pixel>) {
        self.base.render(graphics, buffer);
    }
}

impl <'a, Pixel>OnLoad for Window<'a, Pixel> {
    fn on_load(&mut self) {
        match self.on_load {
            Some(ref mut h) => h.on_load(),
            None => ()
        }
    }
}

impl <'a, Pixel>OnUnload for Window<'a, Pixel> {
    fn on_unload(&mut self) {
        match self.on_unload {
            Some(ref mut h) => h.on_unload(),
            None => ()
        }
    }
}

impl <'a, Pixel>OnEvent for Window<'a, Pixel> {
    fn on_event(&mut self, e: &events::Event) {
        match self.on_event {
            Some(ref mut h) => h.on_event(e),
            None => ()
        }
    }
}

