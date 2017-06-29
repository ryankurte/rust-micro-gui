
use std::*;
use std::sync::mpsc;

use types::*;
use types::events::ID;

extern crate sdl2;
use self::sdl2::*;
use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::PixelFormatEnum;

pub struct Renderer {
    w: u32,
    h: u32,
    context: sdl2::Sdl,
    canvas: render::WindowCanvas,
    pub event_rx: mpsc::Receiver<events::Event>,
    event_tx: mpsc::Sender<events::Event>
}

impl Renderer {
    /// Create a new native (SDL2 based) renderer
    pub fn new(name: &str, w: u32, h: u32) -> Renderer {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window(name, w, h)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().software().build().unwrap();

        let (event_tx, event_rx): (mpsc::Sender<events::Event>, mpsc::Receiver<events::Event>) = mpsc::channel();

        return Renderer{w, h, context, canvas, event_rx, event_tx};
    }

    /// Update should be run in the main loop
    /// This parses SDL events and converts them to ugui events on the event channel
    pub fn update(&mut self) -> bool {
        let mut running = true;

        for event in self.context.event_pump().unwrap().poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } |
                Event::Quit { .. } => { running = false; },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => { self.event_tx.send(events::Event{id: ID::Up, x: 0, y: 0}).unwrap(); },       
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => { self.event_tx.send(events::Event{id: ID::Down, x: 0, y: 0}).unwrap(); },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => { self.event_tx.send(events::Event{id: ID::Left, x: 0, y: 0}).unwrap(); },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => { self.event_tx.send(events::Event{id: ID::Right, x: 0, y: 0}).unwrap(); },
                Event::KeyDown { keycode: Some(Keycode::Return), .. } => { self.event_tx.send(events::Event{id: ID::Select, x: 0, y: 0}).unwrap(); },
                Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => { self.event_tx.send(events::Event{id: ID::Back, x: 0, y: 0}).unwrap(); },        
                _ => {}
            }
        }
        
        return running;
    }

    fn render_common(&mut self, data: &[u8]) {
        let creator = self.canvas.texture_creator();
        let mut texture = creator.create_texture_target(PixelFormatEnum::RGBA8888, self.w, self.h).unwrap();

        texture.update(None, data, (self.w * 4) as usize).unwrap();

        self.canvas.copy_ex(&texture, None, None, 0.0, None, false, false).unwrap();
        self.canvas.present();
    }

    /// Render a black and white image
    #[cfg(not(any(feature="rgb24")))]
    pub fn render(&mut self, pixels: &[u8]) {
        let size = (self.w * self.h * 4) as usize;
        let mut data: Vec<u8> = vec![0; size];

        for y in 0..self.h {
            for x in 0..self.w {
                let pixel_index = (y * self.w + x / 8) as usize;
                let pixel_mask = (x % 8) as u8;
                let data_index = ((y * self.w + x) * 4) as usize;

                if pixels[pixel_index] & pixel_mask == 0 {
                    data[data_index+0] = 0;
                    data[data_index+1] = 0;
                    data[data_index+2] = 0;
                    data[data_index+3] = 0;
                } else {
                    data[data_index+0] = 255;
                    data[data_index+1] = 255;
                    data[data_index+2] = 255;
                    data[data_index+3] = 255;
                }
            }
        }

        self.render_common(data.as_slice()); 
    }

    /// Render an rgb24 (8,8,8) encoded image
    #[cfg(feature="rgb24")]
    pub fn render(&mut self, pixels: &[u8]) {
        let size = (self.w * self.h * 4) as usize;
        let mut data: Vec<u8> = vec![0; size];

        for y in 0..self.h {
            for x in 0..self.w {
                let pixel_index = ((y * self.w + x) * 3) as usize;
                let data_index = ((y * self.w + x) * 4) as usize;

                data[data_index+3] = pixels[pixel_index+0];
                data[data_index+2] = pixels[pixel_index+1];
                data[data_index+1] = pixels[pixel_index+2];
                data[data_index+0] = 255;
            }
        }

        self.render_common(data.as_slice()); 
    }

}

#[cfg(test)]
mod tests {

}