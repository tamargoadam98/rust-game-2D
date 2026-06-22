use minifb::{Key, Window};

pub struct Input {
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            moving_left: false,
            moving_right: false,
            moving_up: false,
            moving_down: false,
        }
    }

    pub fn poll(&mut self, window: &Window) {
        self.moving_left = window.is_key_down(Key::A) || window.is_key_down(Key::Left);
        self.moving_right = window.is_key_down(Key::D) || window.is_key_down(Key::Right);
        self.moving_up = window.is_key_down(Key::W) || window.is_key_down(Key::Up);
        self.moving_down = window.is_key_down(Key::S) || window.is_key_down(Key::Down);
    }

    pub fn is_moving_left(&self) -> bool {
        self.moving_left
    }

    pub fn is_moving_right(&self) -> bool {
        self.moving_right
    }

    pub fn is_moving_up(&self) -> bool {
        self.moving_up
    }

    pub fn is_moving_down(&self) -> bool {
        self.moving_down
    }
}
