use crossterm::event::{Event, KeyCode, KeyEventKind, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::game::game::Game;
use crate::game::util::Direction;

pub struct Controller<'a> {
    pub game: &'a Game<'a>,
}

impl Controller<'_> {
    pub fn set_game(&self, game: &Game) {
        println!("{:?}", game);
    }
    pub fn begin(&self) {
        enable_raw_mode().unwrap(); // raw mode — ввод без Enter

        println!("Нажимай клавиши (q — выход)");

        loop {
            if let Event::Key(event) = read().unwrap() {
                if event.kind != KeyEventKind::Press {
                    continue;
                }

                match event.code {
                    KeyCode::Char('q') => break,
                    // KeyCode::Char(c) => println!("Нажата клавиша: {}", c),
                    KeyCode::Up => {
                        println!("↑");
                        self.game.move_character(Direction::Up);
                    }
                    KeyCode::Down => {
                        println!("↓");
                        self.game.move_character(Direction::Down);
                    }
                    KeyCode::Left => {
                        println!("←");
                        self.game.move_character(Direction::Left);
                    }
                    KeyCode::Right => {
                        println!("→");
                        self.game.move_character(Direction::Right);
                    }
                    _ => {}
                }
            }
        }

        disable_raw_mode().unwrap();
    }
}
