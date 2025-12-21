#[derive(Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Size {
    pub fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

struct GameCharacter {
    position: Position,
}

#[derive(Debug)]
pub struct Game {
    pub field: Size,
    pub level_index: u32,
}

#[derive(Debug)]
pub struct Map {
    pub data: String,
    pub size: Size,
}

impl Map {
    pub fn get_point(&self, position: Position) -> Option<char> {
        self.data
            .chars()
            .nth((position.y * self.size.width as i32 + position.x) as usize)
    }
}
