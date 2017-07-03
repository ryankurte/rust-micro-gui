//! Events for ugui interaction
//!
//! Copyright 2017 Ryan Kurte

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ID {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Click
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Event {
    pub id: ID,
    pub x: usize,
    pub y: usize
}
