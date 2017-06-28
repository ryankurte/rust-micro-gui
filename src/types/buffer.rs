
use types::*;

pub trait BufferSet {
    fn set(&mut self, x: usize, y: usize, p: pixel::Pixel);
}

pub trait BufferGet {
    fn get(&self, x: usize, y: usize) -> pixel::Pixel;
}

