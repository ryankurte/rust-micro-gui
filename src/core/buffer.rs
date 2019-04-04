//! Buffer module creates a read write buffer from a reference to a data array.
//!
//! Copyright 2017 Ryan Kurte

use std::*;

use std::marker::PhantomData;

use types::pixel::{PixelBW, PixelRGB24};

/// Buff trait encompasses methods required for a graphics buffer
pub trait Buff<Pixel> {
    /// Set trait sets a pixel in the buffer
    fn set(&mut self, x: usize, y: usize, p: &Pixel);
    /// Get trait fetches a pixel in a buffer
    fn get(&self, x: usize, y: usize) -> Pixel;
    /// Size trait fetches the size of a buffer (in pixels)
    fn size(&self) -> (usize, usize);
    /// Clear clears the buffer
    fn clear(&mut self, p: &Pixel);
}

/// Buffer implements a generic display buffer over an arbitrary pixel type
#[derive(Debug)]
pub struct Buffer<'a, Pixel> {
    pub width: usize,
    pub height: usize,
    pub porch_bytes: usize,
    pub trailer_bytes: usize,
    pub line_width_bytes: usize,
    pub data: &'a mut [u8],
    _pixel: PhantomData<Pixel>,
}

impl <'a, Pixel> Buffer <'a, Pixel> {
    pub fn data(&self) -> &[u8] {
        self.data
    }
}

impl <'a> Buffer <'a, PixelBW> {
    // Create a new black and white buffer
    pub fn new(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &'a mut [u8]) -> Self {
        let line_width_bytes = porch_bytes + width / 8 + trailer_bytes;
        return Self{width, height, porch_bytes, trailer_bytes, line_width_bytes, data, _pixel: PhantomData}
    }
}

impl <'a> Buff<PixelBW> for Buffer <'a, PixelBW> {
    /// Black and white mode pixel set function
    fn set(&mut self, x: usize, y: usize, p: &PixelBW) {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8) as u8;

        if *p {
            self.data[index as usize] |= mask;
        } else {
            self.data[index as usize] &= !mask;
        }
    }

    /// Black and white mode pixel get function
    fn get(&self, x: usize, y: usize) -> PixelBW {
        let index = self.line_width_bytes * y + self.porch_bytes + x / 8;
        let mask = 1 << (7 - x % 8);

        if (self.data[index as usize] & mask) != 0 {
             return true;
        } else {
            return false;
        }
    }  

    /// Black and White mode buffer clear function
    fn clear(&mut self, p: &PixelBW) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, p);
            }
        }
    } 

    /// Fetch the buffer size in pixels
    fn size(&self) -> (usize, usize) {
        return (self.width, self.height);
    }
}

impl <'a> Buffer <'a, PixelRGB24> {
    // Create a new rgb24 buffer
    pub fn new(width: usize, height: usize, porch_bytes: usize, trailer_bytes: usize, data: &'a mut [u8]) -> Self {
        let line_width_bytes = porch_bytes + width * 3 + trailer_bytes;
        return Self{width, height, porch_bytes, trailer_bytes, line_width_bytes, data, _pixel: PhantomData}
    }
}

impl <'a> Buff<PixelRGB24> for Buffer <'a, PixelRGB24> {

    /// RGB24 mode pixel set function
    fn set(&mut self, x: usize, y: usize, p: &PixelRGB24) {
        let index: usize = self.line_width_bytes * y + x * 3 + self.porch_bytes;
        
        self.data[index + 0] = p.r;
        self.data[index + 1] = p.g;
        self.data[index + 2] = p.b;
    }

    /// RGB24 mode pixel get function
    fn get(&self, x: usize, y: usize) -> PixelRGB24 {
        let index: usize = self.line_width_bytes * y + x * 3 + self.porch_bytes;

        return PixelRGB24{
            r: self.data[index + 0],
            g: self.data[index + 1],
            b: self.data[index + 2]} 
    }

    /// RGB24 mode buffer clear function
    fn clear(&mut self, p: &PixelRGB24) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, p);
            }
        }
    }

    /// Fetch the buffer size in pixels
    fn size(&self) -> (usize, usize) {
        return (self.width, self.height);
    }
}


/// Format implementation for the buffer
impl <'a, Pixel>fmt::Display for Buffer<'a, Pixel> {
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
        let data_slice = &mut data[..];
        let mut buffer = Buffer::<PixelBW>::new(WIDTH, HEIGHT, PORCH, TRAILER, data_slice);

        buffer.set(0, 0, &true);
        assert_eq!(buffer.get(0, 0), true);
        assert_eq!(buffer.data[PORCH], 0x80);

        buffer.set(0, 0, &false);
        assert_eq!(buffer.get(0, 0), false);
        assert_eq!(buffer.data[PORCH], 0x00);

        buffer.set(1, 1, &true);
        assert_eq!(buffer.get(1, 1), true);
        assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x40);

        buffer.set(1, 1, &false);
        assert_eq!(buffer.get(1, 1), false);
        assert_eq!(buffer.data[PORCH + WIDTH / 8 + TRAILER + PORCH], 0x00);
    }

    #[test]
    fn test_rgb24_porch_trailer() {
        let mut data: [u8; SIZE_RGB24] = [0; SIZE_RGB24];
        let data_slice = &mut data[..];

        let blank: [u8; SIZE_RGB24] = [0; SIZE_RGB24];
        let blank_slice = &blank[..];

        let mut buffer = Buffer::<PixelRGB24>::new(WIDTH, HEIGHT, PORCH, TRAILER, data_slice);

        buffer.set(0, 0, &PixelRGB24::white());
        assert_eq!(buffer.get(0, 0), PixelRGB24::white());
        assert_eq!(buffer.data[PORCH + 0], 0xFF);
        assert_eq!(buffer.data[PORCH + 1], 0xFF);
        assert_eq!(buffer.data[PORCH + 2], 0xFF);

        buffer.set(0, 0, &PixelRGB24::black());
        assert_eq!(buffer.get(0, 0), PixelRGB24::black());
        assert_eq!(buffer.data, blank_slice);

        buffer.set(1, 1, &PixelRGB24::white());
        assert_eq!(buffer.get(1, 1), PixelRGB24::white());

        let index: usize = (PORCH + WIDTH * 3 + TRAILER) + 3 + PORCH;
        assert_eq!(buffer.data[index + 0], 0xFF);
        assert_eq!(buffer.data[index + 1], 0xFF);
        assert_eq!(buffer.data[index + 2], 0xFF);

        buffer.set(1, 1, &PixelRGB24::black());
        assert_eq!(buffer.get(1, 1), PixelRGB24::black());
        assert_eq!(buffer.data, blank_slice);
    }

    const X: usize = 2;
    const Y: usize = 2;

    #[test]
    fn test_rgb24_buffer() {
        let mut data = [0u8; X * Y * 3];

        let mut buffer = Buffer::<PixelRGB24>::new(X, Y, 0, 0, &mut data);

        let white = PixelRGB24::white();
        let black = PixelRGB24::black();

        for y in 0..Y {
            for x in 0..X {
                buffer.clear(&white);

                buffer.set(x, y, &black);
                assert_eq!(buffer.get(x, y), black);

                let index = (y * Y + x) * 3;
                assert_eq!(buffer.data[index+0], 0x00);
                assert_eq!(buffer.data[index+1], 0x00);
                assert_eq!(buffer.data[index+2], 0x00);
            }
        }
    }
}

