//! Buffer traits for custom buffer implementations
//!
//! Copyright 2017 Ryan Kurte


use types::*;

pub trait Set {
    fn set(&mut self, x: usize, y: usize, p: &pixel::Pixel);
}

pub trait Get {
    fn get(&self, x: usize, y: usize) -> pixel::Pixel;
}

pub trait Size {
    fn size(&self) -> (usize, usize);
}
