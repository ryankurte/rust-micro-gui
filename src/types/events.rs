
pub enum ID {
    Up,
    Down,
    Left,
    Right,
    Select,
    Back,
    Click
}

pub struct Event {
    pub id: ID,
    pub x: usize,
    pub y: usize
}
