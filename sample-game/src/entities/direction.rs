use std::fmt;

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            Direction::Up => "Up",
            Direction::UpRight => "UpRight",
            Direction::Right => "Right",
            Direction::DownRight => "DownRight",
            Direction::Down => "Down",
            Direction::DownLeft => "DownLeft",
            Direction::Left => "Left",
            Direction::UpLeft => "UpLeft",
        };
        write!(f, "{}", status_str)
    }
}
