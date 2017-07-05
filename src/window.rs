//! Window implements the top level GUI abstraction.
//! Windows can create and consume events, and contain a base layer from which windows can be implemented.
//!
//! Copyright 2017 Ryan Kurte

use std::*;

use types::*;
use layer::*;
use graphics::*;

/// Window object contains renderable layers and handles events
pub struct Window<'a> {
    base: Layer<'a>,
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
    fn on_event(&mut self, e: &events::Event);
}

impl <'a>Window<'a> {
    /// Create a new window for rendering
    pub fn new(w: usize, h: usize, renderer: Option<&'a mut (Renderable + 'a)>) -> Window<'a> {
        let bounds = rect::Rect::new(0, 0, w, h);
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
    pub fn render(&mut self, graphics: &mut Graphics, buffer: &mut buffer::Buff) {
        self.base.render(graphics, buffer);
    }
}

impl <'a>OnLoad for Window<'a> {
    fn on_load(&mut self) {
        match self.on_load {
            Some(ref mut h) => h.on_load(),
            None => ()
        }
    }
}

impl <'a>OnUnload for Window<'a> {
    fn on_unload(&mut self) {
        match self.on_unload {
            Some(ref mut h) => h.on_unload(),
            None => ()
        }
    }
}

impl <'a>OnEvent for Window<'a> {
    fn on_event(&mut self, e: &events::Event) {
        match self.on_event {
            Some(ref mut h) => h.on_event(e),
            None => ()
        }
    }
}

