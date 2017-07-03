//! Render traits for renderable objects
//!
//! Copyright 2017 Ryan Kurte


use types::*;

pub trait Renderable {
    fn get(&mut self, x: usize, y: usize) -> p: &pixel::Pixel;
    fn size(&self) -> (usize, usize);
}