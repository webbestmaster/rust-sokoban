use std::io::stdout;
use crate::game::game::Game;
use crate::game::util::{Position, Size};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};

pub struct Render {}

impl Render {
    pub fn draw(&self, game: &Game) {
        // TODO: uncomment this to clear screen
        self.clear_screen();

        let mut map = game.map.map.chars().collect::<Vec<char>>();
        let width = (game.map.map.lines().next().unwrap().chars().count() + 1) as i32;

        let Position { x, y } = game.map.character.position;

        // draw points
        game.map.point_list.iter().for_each(|point| {
            if let Some(place) = map.get_mut((point.y * width + point.x) as usize) {
                *place = '□';
            };
        });

        game.map.item_list.iter().for_each(|point| {
            if let Some(place) = map.get_mut((point.y * width + point.x) as usize) {
                *place = '◼';
            };
        });

        // draw character
        if let Some(place) = map.get_mut((y * width + x) as usize) {
            *place = '@';
        };
        
        // draw map
        println!("{}", map.iter().collect::<String>());
    }

    pub fn clear_screen(&self) {
        execute!(stdout(), Clear(ClearType::All)).unwrap();
    }
}
