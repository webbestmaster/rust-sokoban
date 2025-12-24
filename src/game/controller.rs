use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::game::game::Game;
use crate::game::render::Render;
use crate::game::util::Direction;

pub struct Controller<'a> {
    pub game: &'a mut Game,
    pub render: &'a Render,
}

impl Controller<'_> {
    pub fn set_game(&self, game: &Game) {
        println!("{:?}", game);
    }

    pub fn begin(&mut self) {
        enable_raw_mode().unwrap(); // raw mode — ввод без Enter

        // println!("Нажимай клавиши (q — выход)");

        loop {
            if let Event::Key(keyEvent) = read().unwrap() {
                if keyEvent.kind != KeyEventKind::Press {
                    continue;
                }

                match keyEvent.code {
                    KeyCode::Char('c') if keyEvent.modifiers.contains(KeyModifiers::CONTROL) => {
                        break;
                    }
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

            self.render.draw(&self.game);
        }

        disable_raw_mode().unwrap();
    }
}
