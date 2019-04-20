//! ugui example application
//!
//! Copyright 2019 Ryan Kurte

extern crate micro_gui;

use micro_gui::native::Renderer;
use micro_gui::prelude::*;
use micro_gui::types::pixel::*;

use micro_gui::widgets::demo::DemoWidget;

extern crate structopt;
use structopt::StructOpt;

pub mod common;

type Pixel = PixelBW;

#[cfg(feature = "sdl")]
fn main() {
    let config = common::Config::from_args();

    let size = config.width * config.height * 3;

    println!("config: {:?}", config);

    // Create backing data
    let mut data = vec![0u8; size];
    let data_slice = &mut data[..];

    // Create buffer to wrap data
    let mut buffer = Buffer::<Pixel>::new(config.width, config.height, 0, 0, data_slice);

    let mut demo = DemoWidget::new();
    let mut window = Window::<Pixel>::new(config.width, config.height, Some(&mut demo));

    // Create gui instance that consumes buffer
    let mut gui = Gui::new(config.width, config.height);

    gui.push_window(&mut window);

    // Native renderer allows local display
    let mut native = Renderer::<Pixel>::new(
        &"Rust micro-gui BW example",
        config.width as u32,
        config.height as u32,
    );

    println!("Rust microgui running!");

    loop {
        buffer.clear(&Pixel::white());
        gui.render(&mut buffer);
        native.render(buffer.data);

        let running = native.update();
        for e in native.event_rx.try_iter() {
            gui.event(&e);
            println!("Event: {:?}", e);
        }

        if !running {
            break;
        }
    }
}
