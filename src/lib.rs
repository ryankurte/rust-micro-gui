
use std::*;

pub mod types;
use types::*;

pub mod layer;
pub mod window;
use window::*;

pub mod buffer;

pub mod graphics;
use graphics::*;

pub mod widgets;

#[cfg(feature = "sdl")]
pub mod native;

/// Top level GUI object
pub struct GUI<'a> {
    graphics: Graphics,
    windows: vec::Vec<&'a mut window::Window<'a>>
}

impl <'a> GUI <'a> {
    /// Create a new GUI instance of the provided size with the specified buffer
    pub fn new(w: usize, h: usize) -> GUI<'a> {
        let graphics = Graphics::new(0, 0, w, h);
        let windows = Vec::new();

        return GUI{graphics, windows};
    }

    /// Push a window to the top of the window stack
    /// This will cause the window to be rendered
    pub fn push_window(&mut self, w: &'a mut window::Window<'a>) {
        w.on_load();
        self.windows.push(w);
    }

    /// Pop a window from the top of the window stack
    /// This will cause the previous window to be re-rendered
    pub fn pop_window(&mut self) -> Option<&mut window::Window<'a>> {
        let mut window = self.windows.pop();
        match window {
            Some(ref mut w) => w.on_unload(),
            None => ()
        }
        return window;
    }

    /// Render will render the current window
    pub fn render(&mut self, buff: &mut types::buffer::Buff) {
        buff.clear();

        let mut windows = self.windows.as_mut_slice();
        let len = windows.len();
        if len <= 0 { return; }

        let mut active = &mut windows[len-1];
        active.render(&mut self.graphics, buff);
    }

    /// Event passes a ugui event to the current window
    pub fn event(&mut self, e: &events::Event) {
        let mut windows = self.windows.as_mut_slice();
        let len = windows.len();
        if len <= 0 { return; }

        let mut active = &mut windows[len-1];
        active.on_event(e);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
