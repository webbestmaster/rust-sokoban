use crate::game::character::Character;
use crate::game::map::Map;
use crate::game::util::{Direction, Position};

#[derive(Debug)]
pub struct Game {
    pub map: Map,
    pub level_index: u32,
}

#[derive(Debug)]
pub struct GameState {
    pub map: Map,
    pub level_index: u32,
    // pub character: Character,
}

impl Game {
    pub fn move_character(&mut self, direction: Direction) {
        // try to move the character
        println!("game, {:?}", direction);

        let current_position = self.map.character.position;

        let delta = match direction {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        };

        let next_position = Position {
            y: current_position.y + delta.y,
            x: current_position.x + delta.x,
        };

        let next_over_position = Position {
            y: next_position.y + delta.y,
            x: next_position.x + delta.x,
        };

        // item + item is next -> stop
        let is_next_item = self.map.get_is_item_here(next_position);
        let is_next_over_item = self.map.get_is_item_here(next_over_position);

        let is_next_wall = self.map.get_is_wall_here(next_position);
        let is_next_over_wall = self.map.get_is_wall_here(next_over_position);

        if is_next_wall {
            return;
        }

        if is_next_item && (is_next_over_item || is_next_over_wall) {
            return;
        }

        if is_next_item && !is_next_over_item && !is_next_over_wall {
            self.map.character.position.x += delta.x;
            self.map.character.position.y += delta.y;

            if let Some(item) = self
                .map
                .item_list
                .iter_mut()
                .find(|position| position.x == next_position.x && position.y == next_position.y)
            {
                item.x += delta.x;
                item.y += delta.y;
            }
            return;
        }

        self.map.character.position.x += delta.x;
        self.map.character.position.y += delta.y;
    }
}
