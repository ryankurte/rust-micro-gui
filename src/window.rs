//! Window implements the top level GUI abstraction.
//! Windows can create and consume events, and contain a base layer from which windows can be implemented.
//!
//! Copyright 2017 Ryan Kurte

use types::*;
use layer::*;

pub struct Window<'a> {
    base: Layer<'a>,
    on_load: Option<&'a mut (OnLoad + 'a)>,
    on_unload: Option<&'a mut (OnUnload + 'a)>,
    on_event: Option<&'a mut (OnEvent + 'a)>
}

pub trait OnLoad {
    fn on_load(&mut self);
}

pub trait OnUnload {
    fn on_unload(&mut self);
}

pub trait OnEvent {
    fn on_event(&mut self, e: &events::Event);
}

impl <'a>Window<'a> {
    pub fn new(w: usize, h: usize) -> Window<'a> {
        let bounds = rect::Rect::new(0, 0, w, h);
        let base = Layer::new(bounds);
        return Window{base: base, on_load: None, on_event: None, on_unload: None}
    }

    pub fn bind_handlers(&mut self, on_load: Option<&'a mut OnLoad>, on_unload: Option<&'a mut OnUnload>, on_event: Option<&'a mut OnEvent>) {
        self.on_load = on_load;
        self.on_unload = on_unload;
        self.on_event = on_event;
    }

    pub fn render(&mut self) {
        self.base.update();
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

