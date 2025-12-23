use crate::game::character::Character;
use crate::game::util::{Position, Size};

#[derive(Debug, Clone)]
pub struct Map<'a> {
    pub map: &'a str,
    pub character: Character,
    pub point_list: &'a [Position],
    pub item_list: &'a [Position],
}

impl Map<'_> {
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
}
