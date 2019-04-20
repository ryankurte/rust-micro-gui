//! Pixel type definitions
//! This is designed so that modules that do not need to interact with pixels directly
//! can use pixel::Pixel or standard pixel traits and be compatible with all pixel representations.
//!
//! Copyright 2019 Ryan Kurte

/// Black and White trait to be implemented by all colours
pub trait BW {
    fn black() -> Self;
    fn white() -> Self;
}

/// RGB trait to be implemented by all colours
pub trait RGB {
    fn red() -> Self;
    fn green() -> Self;
    fn blue() -> Self;
}

/// 24-bit RGB pixel implementation
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct PixelRGB24 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PixelRGB24 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        return Self { r, g, b };
    }

    pub fn from_hex(hex: u32) -> Self {
        return Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: ((hex >> 0) & 0xFF) as u8,
        };
    }

    pub fn nice_red() -> Self {
        Self::from_hex(0xB2000E)
    }
    pub fn nice_blue() -> Self {
        Self::from_hex(0x0009B2)
    }
    pub fn nice_green() -> Self {
        Self::from_hex(0x00B22B)
    }
    pub fn nice_yellow() -> Self {
        Self::from_hex(0xFFD119)
    }
}

impl BW for PixelRGB24 {
    fn black() -> Self {
        Self {
            r: 0x00,
            g: 0x00,
            b: 0x00,
        }
    }
    fn white() -> Self {
        Self {
            r: 0xff,
            g: 0xff,
            b: 0xff,
        }
    }
}

impl RGB for PixelRGB24 {
    fn red() -> Self {
        Self::nice_red()
    }
    fn green() -> Self {
        Self::nice_green()
    }
    fn blue() -> Self {
        Self::nice_blue()
    }
}

/// 8-bit grey-scale pixel implementation
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct PixelG8(u8);

impl BW for PixelG8 {
    fn black() -> Self {
        Self(0xff)
    }
    fn white() -> Self {
        Self(0x00)
    }
}

impl RGB for PixelG8 {
    fn red() -> Self {
        Self(0b0010_0000)
    }
    fn green() -> Self {
        Self(0b0100_0000)
    }
    fn blue() -> Self {
        Self(0b1000_0000)
    }
}

/// 1-bit wlack and white pixel implementation
pub type PixelBW = bool;

impl BW for PixelBW {
    fn black() -> Self {
        true
    }
    fn white() -> Self {
        false
    }
}

impl RGB for PixelBW {
    fn red() -> Self {
        true
    }
    fn green() -> Self {
        true
    }
    fn blue() -> Self {
        true
    }
}
