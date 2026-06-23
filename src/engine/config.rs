pub struct Config {
    pub width: usize,
    pub height: usize,
    pub title: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Self {
            width: 640,
            height: 360,
            title: "Simple Engine",
        }
    }
}
