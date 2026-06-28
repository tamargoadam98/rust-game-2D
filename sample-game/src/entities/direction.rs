use std::fmt;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status_str = match self {
            Direction::Up => "Up",
            Direction::Right => "Right",
            Direction::Down => "Down",
            Direction::Left => "Left",
        };
        write!(f, "{}", status_str)
    }
}
