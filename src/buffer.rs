//! Buffer module creates a read write buffer from a reference to a data array.
//!
//! Copyright 2017 Ryan Kurte

use types::*;
use types::buffer::*;

#[derive(Debug)]
pub struct Buffer<'a> {
    pub width: usize,
    pub height: usize,
    pub porch_bytes: usize,
    pub trailer_bytes: usize,
    pub line_width_bytes: usize,
    data: &'a mut [u8]
}

/// Base buffer implementation
impl <'a>Buffer<'a> {
    pub fn new(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &mut [u8]) -> Buffer {
        let line_width_bytes = porch_bytes + width / 8 + trailer_bytes;
        return Buffer{width, height, porch_bytes, trailer_bytes, line_width_bytes, data}
    }
}

/// Set pixel implementation for the buffer
impl <'a>BufferSet for Buffer<'a> {

    /// B/W mode pixel set function
    #[cfg(not(RGB))]
    fn set(&mut self, x: usize, y: usize, p: pixel::PixelBW) {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8) as u8;

        if p {
            self.data[index as usize] |= mask;
        } else {
            self.data[index as usize] &= !mask;
        }
    }

    /// RGB mode pixel set function
    #[cfg(RGB)]
    fn set(&mut self, x: usize, y: usize, p: pixel::PixelRGB) {

    }
}

/// Get pixel implementation for the buffer
impl <'a>BufferGet for Buffer<'a> {

    /// B/W mode pixel get function
    #[cfg(not(RGB))]
    fn get(&self, x: usize, y: usize) -> pixel::PixelBW {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8);

        if (self.data[index as usize] & mask) != 0 {
             return true;
        } else {
            return false;
        }
    }  

    /// RGB mode pixel set function
    #[cfg(RGB)]
    fn get(&self, x: usize, y: usize) -> pixel::PixelRGB {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 16;
    const HEIGHT: usize = 16;
    const PORCH: usize = 1;
    const TRAILER: usize = 1;
    const SIZE: usize = (PORCH + WIDTH / 8 + TRAILER) * HEIGHT;

    #[test]
    fn test() {
        let mut data: [u8; SIZE] = [0; SIZE];
        let mut data_slice = &mut data[..];
        let mut buffer = Buffer::new(WIDTH, HEIGHT, PORCH, TRAILER, data_slice);

        buffer.set(0, 0, true);
        assert_eq!(buffer.get(0, 0), true);
        assert_eq!(buffer.data[PORCH], 0x80);

        buffer.set(0, 0, false);
        assert_eq!(buffer.get(0, 0), false);
        assert_eq!(buffer.data[PORCH], 0x00);

        buffer.set(1, 1, true);
        assert_eq!(buffer.get(1, 1), true);
        assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x40);

        buffer.set(1, 1, false);
        assert_eq!(buffer.get(1, 1), false);
        assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x00);
    }
}

