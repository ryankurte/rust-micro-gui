//! Native rendering module provides mechanisms for rendering ugui interfaces
//! on standard computers using sdl2.
//!
//! Copyright 2017 Ryan Kurte


use std::*;
use std::sync::mpsc;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::types::*;
use crate::types::events::Event;

extern crate sdl2;
use self::sdl2::*;
use self::sdl2::event::Event as SdlEvent;
use self::sdl2::keyboard::Keycode;
use self::sdl2::pixels::PixelFormatEnum;

/// Renderer implements an SDL2 based micro-gui renderer
pub struct Renderer<P> {
    w: u32,
    h: u32,
    context: sdl2::Sdl,
    canvas: render::WindowCanvas,
    pub event_rx: mpsc::Receiver<events::Event>,
    event_tx: mpsc::Sender<events::Event>,
    prev_buttons: HashSet<mouse::MouseButton>,
    _pixel: PhantomData<P>,
}

impl <P>Renderer<P> {
    /// Create a new native (SDL2 based) renderer
    pub fn new(name: &str, w: u32, h: u32) -> Self {
        let context = sdl2::init().unwrap();
        let video_subsystem = context.video().unwrap();
        let window = video_subsystem
            .window(name, w, h)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().software().build().unwrap();

        let (event_tx, event_rx): (mpsc::Sender<events::Event>, mpsc::Receiver<events::Event>) = mpsc::channel();

        let prev_buttons = HashSet::new();

        return Renderer{w, h, context, canvas, event_rx, event_tx, prev_buttons, _pixel: PhantomData};
    }

    /// Update should be run in the main loop
    /// This parses SDL events and converts them to ugui events on the event channel
    pub fn update(&mut self) -> bool {
        let mut running = true;

        let mut sdl_events = self.context.event_pump().unwrap();

        // Handle keyboard events
        for event in sdl_events.poll_iter() {
            match event {
                SdlEvent::KeyDown { keycode: Some(Keycode::Escape), .. } |
                SdlEvent::Quit { .. } => { running = false; },
                SdlEvent::KeyDown { keycode: Some(Keycode::Up), .. } => { self.event_tx.send(Event::Up).unwrap(); },       
                SdlEvent::KeyDown { keycode: Some(Keycode::Down), .. } => { self.event_tx.send(Event::Down).unwrap(); },
                SdlEvent::KeyDown { keycode: Some(Keycode::Left), .. } => { self.event_tx.send(Event::Left).unwrap(); },
                SdlEvent::KeyDown { keycode: Some(Keycode::Right), .. } => { self.event_tx.send(Event::Right).unwrap(); },
                SdlEvent::KeyDown { keycode: Some(Keycode::Return), .. } => { self.event_tx.send(Event::Select).unwrap(); },
                SdlEvent::KeyDown { keycode: Some(Keycode::Backspace), .. } => { self.event_tx.send(Event::Back).unwrap(); },        
                _ => {}
            }
        }

        // Handle mouse events
        let state = sdl_events.mouse_state();

        // Create a set of pressed Keys.
        let buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_buttons = &buttons - &self.prev_buttons;
        let old_buttons = &self.prev_buttons - &buttons;

        if !new_buttons.is_empty() && old_buttons.is_empty() {
            self.event_tx.send(Event::Click{x: state.x() as usize, y: state.y() as usize}).unwrap();
        }

        self.prev_buttons = buttons;
        
        return running;
    }

    fn render_common(&mut self, data: &[u8]) {
        let creator = self.canvas.texture_creator();
        let mut texture = creator.create_texture_target(PixelFormatEnum::RGBA8888, self.w, self.h).unwrap();

        texture.update(None, data, (self.w * 4) as usize).unwrap();

        self.canvas.copy_ex(&texture, None, None, 0.0, None, false, false).unwrap();
        self.canvas.present();
    }




}


impl Renderer<PixelBW> {
    pub fn render(&mut self, pixels: &[u8]) {
        let data = Self::bw_to_rgba32(self.w as usize, self.h as usize, pixels);

        self.render_common(data.as_slice()); 
    }

    /// Convert a bit-packed black and white representation to RGBA32
    fn bw_to_rgba32(w: usize, h: usize, pixels: &[u8]) -> Vec<u8> {
        let size = w * h * 4;
        let mut data: Vec<u8> = vec![0; size];

        for y in 0..h {
            let i = y * w / 8;
            let row = &pixels[i .. i + w / 8];

            for x in 0..w {
                let pixel_index = x / 8 as usize;
                let pixel_mask = 1 << (7-(x % 8));
                let data_index = (y * w + x) * 4 as usize;

                let m = if row[pixel_index] & pixel_mask == 0 {
                    0xFF
                } else {
                    0x00
                };
                
                for i in 0..4 {
                    data[data_index + i] = m;
                }
            }
        }

        data
    }
}


impl Renderer<PixelRGB24> {
    /// Render an rgb24 (8,8,8) encoded image
    pub fn render(&mut self, pixels: &[u8]) {
        let data = Self::rgb24_to_rgba32(self.w as usize, self.h as usize, &pixels);

        self.render_common(data.as_slice()); 
    }

    /// Convert a bit-packed black and white representation to RGBA32
    fn rgb24_to_rgba32(w: usize, h: usize, pixels: &[u8]) -> Vec<u8> {
        let size = w * h * 4;
        let mut data: Vec<u8> = vec![0; size];

        for y in 0..h {
            for x in 0..w {
                let pixel_index = (y * w + x) * 3;
                let data_index = (y * w + x) * 4;

                data[data_index+0] = 255;
                data[data_index+1] = pixels[pixel_index+2];
                data[data_index+2] = pixels[pixel_index+1];
                data[data_index+3] = pixels[pixel_index+0];
            }
        }

        data
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bw_to_rgba32() {
        let pixels = [
            0b1000_0001, 
            0b0100_0010,
        ];
        let expected = [
            0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00,
            0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF,
        ].to_vec();


        let mapped = Renderer::<PixelBW>::bw_to_rgba32(8, 2, &pixels);

        assert_eq!(expected.len(), mapped.len());
        assert_eq!(expected, mapped);
    }

    #[test]
    fn test_rgb24_to_rgba32() {
        let pixels = [
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 
            0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc,
        ];
        let expected = [
            //0x11, 0x22, 0x33, 0xFF, 0x44, 0x55, 0x66, 0xFF,
            //0x66, 0x77, 0x88, 0xFF, 0x99, 0xaa, 0xbb, 0xFF,
            0xff, 0x33, 0x22, 0x11, 0xff, 0x66, 0x55, 0x44,
            0xff, 0x99, 0x88, 0x77, 0xff, 0xcc, 0xbb, 0xaa,
        ].to_vec();


        let mapped = Renderer::<PixelRGB24>::rgb24_to_rgba32(2, 2, &pixels);

        assert_eq!(expected.len(), mapped.len());
        assert_eq!(expected, mapped);
    }
}