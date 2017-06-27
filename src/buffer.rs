
use types::*;

pub struct Buffer {
    width: i32,
    height: i32,
    porch: i32,
    trailer: i32,
    data: &'static[u8]
}

impl Buffer {
    pub fn new(width: i32, height: i32, porch: i32, trailer: i32, data: &'static[u8]) -> Buffer {
        return Buffer{width, height, porch, trailer, data}
    }

    pub fn set(x: i32, y: i32, p: pixel::Pixel) {

    }

    pub fn get(x: i32, y: i32) -> pixel::Pixel {

    }
}