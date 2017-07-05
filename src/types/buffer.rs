//! Buffer traits for custom buffer implementations
//!
//! Copyright 2017 Ryan Kurte


use types::*;

/// Buff trait encompasses methods required by implementing buffers
pub trait Buff {
    /// Set trait sets a pixel in the buffer
    fn set(&mut self, x: usize, y: usize, p: &pixel::Pixel);
    /// Get trait fetches a pixel in a buffer
    fn get(&self, x: usize, y: usize) -> pixel::Pixel;
    /// Size trait fetches the size of a buffer (in pixels)
    fn size(&self) -> (usize, usize);
    /// Clear clears the buffer
    fn clear(&mut self);
}
