//! Point represents a point in 2d space
//!
//! Copyright 2017 Ryan Kurte

/// Point in 2d space
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point { Point{x, y} }
}

