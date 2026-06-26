use crate::game::Game;
use crate::input::Action;

pub enum Screen {
    Menu,
    Playing(Game),
    Paused(Game),
    GameOver { score: u32 },
}

pub struct App {
    pub screen: Screen,
    pub quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Menu,
            quit: false,
        }
    }

    pub fn handle_action(&mut self, action: Action) {
        if action == Action::Quit {
            self.quit = true;
            return;
        }

        let screen = std::mem::replace(&mut self.screen, Screen::Menu);
        self.screen = match screen {
            Screen::Menu if action == Action::Select => Screen::Playing(Game::new()),
            Screen::Playing(game) if action == Action::Pause => Screen::Paused(game),
            Screen::Playing(mut game) => {
                apply_game_action(&mut game, action);
                Screen::Playing(game)
            }
            Screen::Paused(game) if matches!(action, Action::Pause | Action::Select) => {
                Screen::Playing(game)
            }
            Screen::Paused(game) => Screen::Paused(game),
            Screen::GameOver { .. } if action == Action::Select => Screen::Menu,
            other => other,
        };
    }

    pub fn tick(&mut self) {
        if let Screen::Playing(game) = &mut self.screen {
            game.tick();
            if game.game_over {
                let score = game.score;
                self.screen = Screen::GameOver { score };
            }
        }
    }
}

fn apply_game_action(game: &mut Game, action: Action) {
    match action {
        Action::MoveLeft => game.move_left(),
        Action::MoveRight => game.move_right(),
        Action::RotateCW => game.rotate_cw(),
        Action::RotateCCW => game.rotate_ccw(),
        Action::SoftDrop => game.soft_drop(),
        Action::HardDrop => game.hard_drop(),
        _ => {}
    }
}
