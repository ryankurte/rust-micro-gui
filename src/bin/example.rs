//! ugui example application
//!
//! Copyright 2017 Ryan Kurte


extern crate micro_gui;

use micro_gui::GUI;
use micro_gui::buffer::Buffer;
use micro_gui::native::Renderer;
use micro_gui::window::Window;
use micro_gui::widgets::demo::DemoWidget;

const WIDTH: usize = 640;
const HEIGHT: usize = 480; 
const SIZE: usize = WIDTH * HEIGHT * 3;

#[cfg(feature = "sdl")]
fn main() {

    // Create backing data
    let mut data: [u8; SIZE] = [0xff; SIZE];
    let mut data_slice = &mut data[..];

    // Create buffer to wrap data
    let mut buffer = Buffer::new(WIDTH, HEIGHT, 0, 0, data_slice);

    let mut demo = DemoWidget::new();
    let mut window = Window::new(WIDTH, HEIGHT, Some(&mut demo));

    // Create gui instance that consumes buffer
    let mut gui = GUI::new(WIDTH, HEIGHT);

    gui.push_window(&mut window);

    

    // Native renderer allows local display
    let mut native = Renderer::new(&"Rust microgui example", WIDTH as u32, HEIGHT as u32);

    println!("Rust microgui running!");

    loop {
        gui.render(&mut buffer);
        native.render(buffer.data);

        let running = native.update();
        for e in native.event_rx.try_iter() {
            gui.event(&e);
            println!("Event: {:?}", e);
        }

        if !running { break; }
    }
}
