use winit::keyboard::{Key, NamedKey};

#[derive(Default)]
pub struct Input {
    moving_left: bool,
    moving_right: bool,
    moving_up: bool,
    moving_down: bool,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_down(&mut self, key: &Key) {
        match key {
            Key::Character(c) if c.as_str() == "a" => self.moving_left = true,
            Key::Character(c) if c.as_str() == "d" => self.moving_right = true,
            Key::Character(c) if c.as_str() == "w" => self.moving_up = true,
            Key::Character(c) if c.as_str() == "s" => self.moving_down = true,
            Key::Named(NamedKey::ArrowLeft) => self.moving_left = true,
            Key::Named(NamedKey::ArrowRight) => self.moving_right = true,
            Key::Named(NamedKey::ArrowUp) => self.moving_up = true,
            Key::Named(NamedKey::ArrowDown) => self.moving_down = true,
            _ => {}
        }
    }

    pub fn key_up(&mut self, key: &Key) {
        match key {
            Key::Character(c) if c.as_str() == "a" => self.moving_left = false,
            Key::Character(c) if c.as_str() == "d" => self.moving_right = false,
            Key::Character(c) if c.as_str() == "w" => self.moving_up = false,
            Key::Character(c) if c.as_str() == "s" => self.moving_down = false,
            Key::Named(NamedKey::ArrowLeft) => self.moving_left = false,
            Key::Named(NamedKey::ArrowRight) => self.moving_right = false,
            Key::Named(NamedKey::ArrowUp) => self.moving_up = false,
            Key::Named(NamedKey::ArrowDown) => self.moving_down = false,
            _ => {}
        }
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
