//! Buffer module creates a read write buffer from a reference to a data array.
//!
//! Copyright 2017 Ryan Kurte

use std::*;

use types::*;

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
#[allow(dead_code)]
impl <'a>Buffer<'a> {
    // Create a new black and white buffer
    fn new_bw(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &mut [u8]) -> Buffer {
        let line_width_bytes = porch_bytes + width / 8 + trailer_bytes;
        return Buffer{width, height, porch_bytes, trailer_bytes, line_width_bytes, data}
    }

    // Create a new rgb24 buffer
    fn new_rgb24(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &mut [u8]) -> Buffer {
        let line_width_bytes = porch_bytes + width * 3 + trailer_bytes;
        return Buffer{width, height, porch_bytes, trailer_bytes, line_width_bytes, data}
    }

    /// Map new to the appropriate buffer type
    #[cfg(not(any(feature="rgb24")))]
    pub fn new(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &mut [u8]) -> Buffer { Self::new_bw(width, height, porch_bytes, trailer_bytes, data) }
    #[cfg(feature="rgb24")]
    pub fn new(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &mut [u8]) -> Buffer { Self::new_rgb24(width, height, porch_bytes, trailer_bytes, data) }

    /// Black and white mode pixel set function
    fn set_bw(&mut self, x: usize, y: usize, p: &pixel::PixelBW) {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8) as u8;

        if *p {
            self.data[index as usize] |= mask;
        } else {
            self.data[index as usize] &= !mask;
        }
    }

    /// RGB24 mode pixel set function
    fn set_rgb24(&mut self, x: usize, y: usize, p: &pixel::PixelRGB24) {
        let index: usize = self.line_width_bytes * y + x * 3 + self.porch_bytes;
        
        self.data[index + 0] = p.r;
        self.data[index + 1] = p.g;
        self.data[index + 2] = p.b;
    }

    /// Black and white mode pixel get function
    fn get_bw(&self, x: usize, y: usize) -> pixel::PixelBW {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8);

        if (self.data[index as usize] & mask) != 0 {
             return true;
        } else {
            return false;
        }
    }  

    /// RGB24 mode pixel get function
    fn get_rgb24(&self, x: usize, y: usize) -> pixel::PixelRGB24 {
        let index: usize = self.line_width_bytes * y + x * 3 + self.porch_bytes;

        return pixel::PixelRGB24{r: self.data[index + 0],
                                 g: self.data[index + 1],
                                 b: self.data[index + 2]} 
    }
}

/// Set pixel implementation for the buffer
impl <'a>buffer::Set for Buffer<'a> {
    /// B/W mode pixel set function
    #[cfg(not(any(feature="rgb24")))]
    fn set(&mut self, x: usize, y: usize, p: &pixel::PixelBW) { self.set_bw(x, y, p) }

    /// RGB mode pixel set function
    #[cfg(feature="rgb24")]
    fn set(&mut self, x: usize, y: usize, p: &pixel::PixelRGB24) { self.set_rgb24(x, y, p) }
}

/// Get pixel implementation for the buffer
impl <'a>buffer::Get for Buffer<'a> {
    /// B/W mode pixel get function
    #[cfg(not(any(feature="rgb24")))]
    fn get(&self, x: usize, y: usize) -> pixel::PixelBW { self.get_bw(x, y) }  

    /// RGB mode pixel set function
    #[cfg(feature="rgb24")]
    fn get(&self, x: usize, y: usize) -> pixel::PixelRGB24 { self.get_rgb24(x, y) }
}

/// Format implementation for the buffer
impl <'a>fmt::Display for Buffer<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { 
        write!(f, "[width: {}px height: {}px porch: {}B trailer: {}B line_width: {}B Data:\n", 
        self.width, self.height, self.porch_bytes, self.trailer_bytes, self.line_width_bytes).unwrap();
        for l in 0..self.height {
            let start: usize = l * self.line_width_bytes;
            let end:   usize = start + self.line_width_bytes - 1;
            write!(f, "\t\t{:?}\n", &self.data[start..end]).unwrap();
        }
        write!(f, "\t]\n").unwrap();
        Ok(())
     }
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
        let mut buffer = Buffer::new_bw(WIDTH, HEIGHT, PORCH, TRAILER, data_slice);

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

        let blank: [u8; SIZE_RGB24] = [0; SIZE_RGB24];
        let blank_slice = &blank[..];

        let mut buffer = Buffer::new_rgb24(WIDTH, HEIGHT, PORCH, TRAILER, data_slice);

        buffer.set_rgb24(0, 0, &PixelRGB24::white());
        assert_eq!(buffer.get_rgb24(0, 0), PixelRGB24::white());
        assert_eq!(buffer.data[PORCH + 0], 0xFF);
        assert_eq!(buffer.data[PORCH + 1], 0xFF);
        assert_eq!(buffer.data[PORCH + 2], 0xFF);

        buffer.set_rgb24(0, 0, &PixelRGB24::black());
        assert_eq!(buffer.get_rgb24(0, 0), PixelRGB24::black());
        assert_eq!(buffer.data, blank_slice);

        buffer.set_rgb24(1, 1, &PixelRGB24::white());
        assert_eq!(buffer.get_rgb24(1, 1), PixelRGB24::white());

        let index: usize = (PORCH + WIDTH * 3 + TRAILER) + 3 + PORCH;
        assert_eq!(buffer.data[index + 0], 0xFF);
        assert_eq!(buffer.data[index + 1], 0xFF);
        assert_eq!(buffer.data[index + 2], 0xFF);

        buffer.set_rgb24(1, 1, &PixelRGB24::black());
        assert_eq!(buffer.get_rgb24(1, 1), PixelRGB24::black());
        assert_eq!(buffer.data, blank_slice);
    }
}

