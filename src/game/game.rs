use crate::game::character::Character;
use crate::game::map::Map;
use crate::game::util::{Direction};

#[derive(Debug)]
pub struct Game<'a> {
    pub map: Map<'a>,
    pub level_index: u32,
}

#[derive(Debug)]
pub struct GameState<'a> {
    pub map: Map<'a>,
    pub level_index: u32,
    // pub character: Character,
}

impl Game<'_> {
    pub fn move_character(&self, direction: Direction) {
        // try to move the character
        println!("game, {:?}", direction);
    }

    pub fn get_state(&'_ self) -> GameState<'_> {
        GameState {
            map: self.map.clone(),
            level_index: self.level_index,
        }

        // return current state
    }
}
