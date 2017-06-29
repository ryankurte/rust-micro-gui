extern crate microgui;

use microgui::types::*;
use microgui::types::pixel::*;
use microgui::buffer::Buffer;
use microgui::native::Renderer;
use microgui::graphics::Graphics;

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

    // Create graphics rendering interface
    let graphics = Graphics::new(0, 0, WIDTH, HEIGHT);

    //graphics.draw_line(&mut buffer, point::Point{x: 0, y: 0}, point::Point{x: WIDTH, y: HEIGHT}, &pixel::Pixel::black());
    //graphics.draw_line(&mut buffer, point::Point{x: 0, y: HEIGHT}, point::Point{x: WIDTH, y: 0}, &pixel::Pixel::black());

    let ellipse_r = WIDTH/4;
    graphics.draw_ellipse(&mut buffer, rect::Rect{x: (WIDTH-ellipse_r)/3, y: (HEIGHT-ellipse_r)/2, w: ellipse_r, h: ellipse_r}, &pixel::Pixel::red());
    graphics.draw_ellipse(&mut buffer, rect::Rect{x: (WIDTH-ellipse_r)/2, y: (HEIGHT-ellipse_r)/2, w: ellipse_r, h: ellipse_r}, &pixel::Pixel::green());
    graphics.draw_ellipse(&mut buffer, rect::Rect{x: (WIDTH-ellipse_r)/3*2, y: (HEIGHT-ellipse_r)/2, w: ellipse_r, h: ellipse_r}, &pixel::Pixel::blue());

    // Native renderer allows local display
    let mut renderer = Renderer::new(&"Rust microgui example", WIDTH as u32, HEIGHT as u32);

    println!("Rust microgui running");

    loop {
        renderer.render(buffer.data);
        let running = renderer.update();

        if !running { break; }
    }

}