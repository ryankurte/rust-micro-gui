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
    pub data: &'a mut [u8]
}

/// Base buffer implementation
/// This implements all supported methods, then maps to types::buffer::{Set, Get} for actual implementations.
impl <'a>Buffer<'a> {
    pub fn new(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &mut [u8]) -> Buffer {
        let line_width_bytes = porch_bytes + width / 8 + trailer_bytes;
        return Buffer{width, height, porch_bytes, trailer_bytes, line_width_bytes, data}
    }

    /// Black and white mode pixel set function
    pub fn set_bw(&mut self, x: usize, y: usize, p: &pixel::PixelBW) {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8) as u8;

        if *p {
            self.data[index as usize] |= mask;
        } else {
            self.data[index as usize] &= !mask;
        }
    }

    /// RGB24 mode pixel set function
    pub fn set_rgb24(&mut self, x: usize, y: usize, p: &pixel::PixelRGB24) {
        let index: usize = (self.line_width_bytes * y + x) * 3 + self.porch_bytes;
        
        self.data[index + 0] = p.r;
        self.data[index + 1] = p.g;
        self.data[index + 2] = p.b;
    }

    /// Black and white mode pixel get function
    pub fn get_bw(&self, x: usize, y: usize) -> pixel::PixelBW {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8);

        if (self.data[index as usize] & mask) != 0 {
             return true;
        } else {
            return false;
        }
    }  

    /// RGB24 mode pixel get function
    pub fn get_rgb24(&self, x: usize, y: usize) -> pixel::PixelRGB24 {
        let index: usize = (self.line_width_bytes * y + x) * 3 + self.porch_bytes;

        return pixel::PixelRGB24{r: self.data[index + 0],
                                 g: self.data[index + 1],
                                 b: self.data[index + 2]} 
    }
}

/// Set pixel implementation for the buffer
impl <'a>Set for Buffer<'a> {
    /// B/W mode pixel set function
    #[cfg(not(any(feature="rgb24")))]
    fn set(&mut self, x: usize, y: usize, p: &pixel::PixelBW) { self.set_bw(x, y, p) }

    /// RGB mode pixel set function
    #[cfg(feature="rgb24")]
    fn set(&mut self, x: usize, y: usize, p: &pixel::PixelRGB24) { self.set_rgb24(x, y, p) }
}

/// Get pixel implementation for the buffer
impl <'a>Get for Buffer<'a> {
    /// B/W mode pixel get function
    #[cfg(not(any(feature="rgb24")))]
    fn get(&self, x: usize, y: usize) -> pixel::PixelBW { self.get_bw(x, y) }  

    /// RGB mode pixel set function
    #[cfg(feature="rgb24")]
    fn get(&self, x: usize, y: usize) -> pixel::PixelRGB24 { self.get_rgb24(x, y) }
}

#[cfg(test)]
mod tests {
    use super::*;

    use types::pixel::*;

    const WIDTH: usize = 16;
    const HEIGHT: usize = 16;
    const PORCH: usize = 1;
    const TRAILER: usize = 1;
    const SIZE_BW: usize = (PORCH + WIDTH / 8 + TRAILER) * HEIGHT;
    const SIZE_RGB24: usize = (PORCH + WIDTH * 3 + TRAILER) * HEIGHT;

    #[test]
    fn test_bw() {
        let mut data: [u8; SIZE_BW] = [0; SIZE_BW];
        let mut data_slice = &mut data[..];
        let mut buffer = Buffer::new(WIDTH, HEIGHT, PORCH, TRAILER, data_slice);

        buffer.set_bw(0, 0, &true);
        assert_eq!(buffer.get_bw(0, 0), true);
        assert_eq!(buffer.data[PORCH], 0x80);

        buffer.set_bw(0, 0, &false);
        assert_eq!(buffer.get_bw(0, 0), false);
        assert_eq!(buffer.data[PORCH], 0x00);

        buffer.set_bw(1, 1, &true);
        assert_eq!(buffer.get_bw(1, 1), true);
        assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x40);

        buffer.set_bw(1, 1, &false);
        assert_eq!(buffer.get_bw(1, 1), false);
        assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x00);
    }

    #[test]
    fn test_rgb24() {
        let mut data: [u8; SIZE_RGB24] = [0; SIZE_RGB24];
        let mut data_slice = &mut data[..];
        let mut buffer = Buffer::new(WIDTH, HEIGHT, PORCH, TRAILER, data_slice);

        buffer.set_rgb24(0, 0, &PixelRGB24::black());
        assert_eq!(buffer.get_rgb24(0, 0), PixelRGB24::black());
        //assert_eq!(buffer.data[PORCH], 0x80);

        buffer.set_rgb24(0, 0, &PixelRGB24::white());
        assert_eq!(buffer.get_rgb24(0, 0), PixelRGB24::white());
        //assert_eq!(buffer.data[PORCH], 0x00);

        buffer.set_rgb24(1, 1, &PixelRGB24::black());
        assert_eq!(buffer.get_rgb24(1, 1), PixelRGB24::black());
        //assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x40);

        buffer.set_rgb24(1, 1, &PixelRGB24::white());
        assert_eq!(buffer.get_rgb24(1, 1), PixelRGB24::white());
        //assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x00);
    }
}

