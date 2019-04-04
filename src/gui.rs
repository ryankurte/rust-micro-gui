
use std::vec::Vec;

use crate::types::*;
use crate::core::buffer::{Buff};
use crate::core::window::{Window, OnEvent, OnLoad, OnUnload};

use crate::graphics::Graphics;

/// Top level Gui object
pub struct Gui<'a, Pixel> {
    graphics: Graphics<Pixel>,
    windows: Vec<&'a mut Window<'a, Pixel>>
}

impl <'a, Pixel> Gui <'a, Pixel> {
    /// Create a new Gui instance of the provided size with the specified buffer
    pub fn new(w: usize, h: usize) -> Self {
        let graphics = Graphics::new(0, 0, w, h);
        let windows = Vec::new();

        return Self{graphics, windows};
    }

    /// Push a window to the top of the window stack
    /// This will cause the window to be rendered
    pub fn push_window(&mut self, w: &'a mut Window<'a, Pixel>) {
        w.on_load();
        self.windows.push(w);
    }

    /// Pop a window from the top of the window stack
    /// This will cause the previous window to be re-rendered
    pub fn pop_window(&mut self) -> Option<&mut Window<'a, Pixel>> {
        let mut window = self.windows.pop();
        match window {
            Some(ref mut w) => w.on_unload(),
            None => ()
        }
        return window;
    }

    /// Render will render the current window
    pub fn render(&mut self, buff: &mut Buff<Pixel>) {
        let windows = self.windows.as_mut_slice();
        let len = windows.len();
        if len <= 0 { return; }

        let active = &mut windows[len-1];
        active.render(&mut self.graphics, buff);
    }

    /// Event passes a ugui event to the current window
    pub fn event(&mut self, e: &events::Event) {
        let windows = self.windows.as_mut_slice();
        let len = windows.len();
        if len <= 0 { return; }

        let active = &mut windows[len-1];
        active.on_event(e);
    }
}
