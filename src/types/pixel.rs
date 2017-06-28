//! Types module defines types for use across ugui implementation
//! Pixel type is abstract and redefined based on requirements
//! This is designed so that modules that do not need to interact with pixels directly
//! can use pixel::Pixel and be compatible with all pixel representations
//!
//! Copyright 2017 Ryan Kurte

/// Generic (ish) RGB pixel implementation
pub struct PixelRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

/// Black and white pixel implementation
pub type PixelBW = bool;

// Magic mapping so pixel::Pixel should be usable everywhere else

// Map RGB pixels if configured
#[cfg(RGB)]
pub type Pixel = PixelRGB;

// Map B/W pixels if RGB not enabled
#[cfg(not(RGB))]
pub type Pixel = PixelBW;

/// RGB16 implementation
#[cfg(RGB16)]
impl PixelRGB {

}

/// RGB24 implementation
#[cfg(RGB24)]
impl PixelRGB {

}
