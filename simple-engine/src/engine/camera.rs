pub struct Camera {
    pub x: i32,
    pub y: i32,
    scroll_margin: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            scroll_margin: 0.0,
        }
    }
}

impl Camera {
    pub fn new(scroll_margin: f32) -> Self {
        Self {
            x: 0,
            y: 0,
            scroll_margin,
        }
    }

    /// Moves the camera to keep `target` within `scroll_margin` pixels of each screen edge.
    pub fn follow(&mut self, target_x: f32, target_y: f32, screen_w: f32, screen_h: f32) {
        let screen_x =
            (target_x - self.x as f32).clamp(self.scroll_margin, screen_w - self.scroll_margin);
        let screen_y =
            (target_y - self.y as f32).clamp(self.scroll_margin, screen_h - self.scroll_margin);
        self.x = (target_x - screen_x) as i32;
        self.y = (target_y - screen_y) as i32;
    }
}
