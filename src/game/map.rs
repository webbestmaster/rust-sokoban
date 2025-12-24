use crate::game::character::Character;
use crate::game::util::{Position, Size};

#[derive(Debug)]
pub struct Map {
    pub map: String,
    pub character: Character,
    pub point_list: Vec<Position>,
    pub item_list: Vec<Position>,
}

pub struct MapStatic<'a> {
    pub map: &'a str,
    pub character: Character,
    pub point_list: &'a [Position],
    pub item_list: &'a [Position],
}

impl Map {
    pub fn get_size(&self) -> Size {
        let width = (self.map.lines().next().unwrap().chars().count() + 1) as i32;
        let height = self.map.lines().count() as i32;

        Size { height, width }
    }
    pub fn get_char(&self, position: Position) -> Option<char> {
        let Size { width, height } = self.get_size();

        self.map
            .chars()
            .nth((position.y * width + position.x) as usize)
    }

    pub fn get_is_wall_here(&self, position: Position) -> bool {
        let Size { width, height } = self.get_size();

        let letter = if let Some(letter) = self
            .map
            .chars()
            .nth((position.y * width + position.x) as usize)
        {
            letter
        } else {
            return false;
        };

        letter == '#'
    }

    pub fn get_is_item_here(&self, position: Position) -> bool {
        self.item_list
            .iter()
            .any(|item| -> bool { item.x == position.x && item.y == position.y })
    }

    pub fn get_is_point_here(&self, position: Position) -> bool {
        self.point_list
            .iter()
            .any(|item| -> bool { item.x == position.x && item.y == position.y })
    }

    pub fn to_map(map: MapStatic) -> Map {
        Map {
            map: map.map.to_string(),
            item_list: Vec::from(map.item_list),
            point_list: Vec::from(map.point_list),
            character: map.character.clone(),
        }
    }
}
