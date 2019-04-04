//! Events for ugui interaction
//!
//! Copyright 2017 Ryan Kurte

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Event {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Click{
        x: usize,
        y: usize,
    },
}

