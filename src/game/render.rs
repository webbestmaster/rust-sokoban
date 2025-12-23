use crate::game::game::Game;
use std::io::{self, Write};

use crate::game::util::{Position, Size};
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

        let mut map = game.map.map.chars().collect::<Vec<char>>();
        let width = (game.map.map.lines().next().unwrap().chars().count() + 1) as i32;

        let Position { x, y } = game.map.character.position;

        // draw character
        if let Some(x) = map.get_mut((y * width + x) as usize) {
            *x = '@';
        };

        // draw points




        // draw items

        // draw map
        println!("{}", map.iter().collect::<String>());
    }

    pub fn clear_screen(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
}
