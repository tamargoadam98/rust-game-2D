#[derive(Copy, Clone)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 800,
            title: "Simple Engine",
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
