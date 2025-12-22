use crate::game::util::{Position, Size};

#[derive(Debug, Copy, Clone)]
pub struct Map<'a> {
    pub data: &'a str,
    pub size: Size,
}

impl Map<'_> {
    pub fn get_point(&self, position: Position) -> Option<char> {
        self.data
            .chars()
            .nth((position.y * self.size.width + position.x) as usize)
    }
}
