
use std::*;

pub mod types;
use types::*;

pub mod layer;
pub mod window;
use window::*;

pub mod graphics;
pub mod buffer;

#[cfg(feature = "sdl")]
pub mod native;

pub struct GUI {
    windows: vec::Vec<&'static mut window::Window<'static>>
}

impl GUI {
    pub fn new(w: usize, h: usize) -> GUI {
        let windows = Vec::new();

        return GUI{windows: windows};
    }

    pub fn push(&mut self, w: &'static mut window::Window) {
        w.on_load();
        self.windows.push(w);
    }

    pub fn pop(&mut self) {
        match self.windows.pop() {
            Some(ref mut w) => w.on_unload(),
            None => ()
        }
    }

    pub fn render(&mut self) {
        if self.windows.len() == 0 {
            return;
        }

        let mut windows = self.windows.as_mut_slice();
        let len = windows.len() - 1;
        let mut active = &mut windows[len];
        active.render();
    }

    pub fn send(&mut self, e: &events::Event) {
        let mut windows = self.windows.as_mut_slice();
        let len = windows.len() - 1;
        let mut active = &mut windows[len];
        active.on_event(e);
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
