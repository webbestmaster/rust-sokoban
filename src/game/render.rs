use std::io::{self, Write};
use crate::game::game::Game;


use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;


pub struct Render {}

impl Render {
    pub fn draw(&self, game: &Game) {
        // TODO: uncomment this to clear screen
        // self.clear_screen();

        let state = game.get_state();

        println!("{}", state.map.data);
    }

    pub fn clear_screen(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
}
