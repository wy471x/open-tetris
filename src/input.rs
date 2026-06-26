use crossterm::event::{KeyCode, KeyEvent};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    MoveLeft,
    MoveRight,
    RotateCW,
    RotateCCW,
    SoftDrop,
    HardDrop,
    Hold,
    Pause,
    Quit,
    Select, // Enter key for menu navigation
}

pub fn map_key(event: KeyEvent) -> Option<Action> {
    match event.code {
        KeyCode::Left => Some(Action::MoveLeft),
        KeyCode::Right => Some(Action::MoveRight),
        KeyCode::Up => Some(Action::RotateCW),
        KeyCode::Down => Some(Action::SoftDrop),
        KeyCode::Char('z') | KeyCode::Char('Z') => Some(Action::RotateCCW),
        KeyCode::Char(' ') => Some(Action::HardDrop),
        KeyCode::Char('c') | KeyCode::Char('C') => Some(Action::Hold),
        KeyCode::Char('p') | KeyCode::Char('P') => Some(Action::Pause),
        KeyCode::Char('q') | KeyCode::Char('Q') => Some(Action::Quit),
        KeyCode::Enter => Some(Action::Select),
        _ => None,
    }
}
