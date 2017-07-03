//! Types module defines types for use across ugui implementation
//!
//! Copyright 2017 Ryan Kurte

/// Rectangle in 2d space
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize
}

impl Rect {
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Rect { Rect{x, y, w, h} }
}
