


use types::*;

pub trait Update {
    fn update(&mut self) -> bool;
}

pub struct Layer<'a> {
    bounds: rect::Rect,
    dirty: bool,
    visible: bool,
    update: Option<&'a mut (Update + 'a)>
}


impl <'a>Layer<'a> {
    pub fn new(bounds: rect::Rect) -> Layer<'a> {
        return Layer{bounds: bounds, dirty: false, visible: true, update: None};
    }

    pub fn bounds(&self) -> rect::Rect {
        return self.bounds;
    }

    pub fn bind_update(&mut self, update: &'a mut Update) {
        self.update = Some(update);
    }

    pub fn set_dirty(&mut self) {
        self.dirty = true;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn update(&mut self) {
        if !self.visible {
            return;
        }
        self.dirty = match self.update {
            Some(ref mut u) => u.update(),
            None => false
        }
    }
}
