pub enum Action {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Exit,
}

impl Action {
    pub fn as_str(&self) -> &'static str {
        match self {
            Action::MoveLeft => "move_left",
            Action::MoveRight => "move_right",
            Action::MoveUp => "move_up",
            Action::MoveDown => "move_down",
            Action::Exit => "exit",
        }
    }
}
