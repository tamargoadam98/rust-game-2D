#[derive(Copy, Clone)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Self {
            width: 1280,
            height: 800,
            title: "Simple Engine",
        }
    }
}
