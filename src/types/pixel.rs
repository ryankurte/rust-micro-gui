//! Pixel type definitions
//! This is designed so that modules that do not need to interact with pixels directly
//! can use pixel::Pixel or standard pixel traits and be compatible with all pixel representations.
//!
//! Copyright 2017 Ryan Kurte

/// Black and White trait to be implemented by all colours
pub trait BW {
    fn black() -> Self;
    fn white() -> Self;
}

/// RGB trait to be implemented by RGB colour formats
pub trait RGB {
    fn red()   -> Self;
    fn green() -> Self;
    fn blue()  -> Self;
}

/// RGB24 pixel implementation
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct PixelRGB24 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl BW for PixelRGB24 {
    fn black() -> PixelRGB24 { PixelRGB24{r: 0x00, g: 0x00, b: 0x00} }
    fn white() -> PixelRGB24 { PixelRGB24{r: 0xff, g: 0xff, b: 0xff} }
}

impl RGB for PixelRGB24 {
    fn red()   -> PixelRGB24 { PixelRGB24{r: 0xff, g: 0x00, b: 0x00} }
    fn green() -> PixelRGB24 { PixelRGB24{r: 0x00, g: 0xff, b: 0x00} }
    fn blue()  -> PixelRGB24 { PixelRGB24{r: 0x00, g: 0x00, b: 0xff} }
}


/// Black and white pixel implementation
pub type PixelBW = bool;

impl BW for PixelBW {
    fn black() -> PixelBW { true }
    fn white() -> PixelBW { false }
}

// Magic mapping so pixel::Pixel should be usable everywhere else

// Map RGB pixels if configured
#[cfg(feature="rgb24")]
pub type Pixel = PixelRGB24;

// Map B/W pixels if RGB not enabled
#[cfg(not(any(feature="rgb24")))]
pub type Pixel = PixelBW;

