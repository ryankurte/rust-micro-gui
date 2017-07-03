


use types::*;

pub struct Layer {
    bounds: rect::Rect,
    dirty: bool,
    visible: bool
}

impl Layer {
    pub fn new(bounds: rect::Rect) -> Layer {
        return Layer{bounds: bounds, dirty: false, visible: true};
    }

    pub fn bounds(&self) -> rect::Rect {
        return self.bounds;
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}
