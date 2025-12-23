use crate::game::util::{Position, Size};

#[derive(Debug, Clone)]
pub struct Map<'a> {
    pub data: &'a str,
    pub start: Position,
    pub point_list: &'a [Position],
    pub item_list: &'a [Position],
    pub size: Size,
}

impl Map<'_> {
    pub fn get_point(&self, position: Position) -> Option<char> {
        self.data
            .chars()
            .nth((position.y * self.size.width + position.x) as usize)
    }
}
