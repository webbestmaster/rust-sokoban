use crossterm::event::{read, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

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

impl Game {
    // TODO: add another move type
    pub fn move_up(&self) {
        // try to move up character
    }

    pub fn get_state(&self) {
        // return current state
    }

    // TODO: make separated module, and call Game's method from it
    pub fn begin(&self) {
        enable_raw_mode().unwrap(); // raw mode — ввод без Enter

        println!("Нажимай клавиши (q — выход)");

        loop {
            if let Event::Key(event) = read().unwrap() {
                if event.kind != KeyEventKind::Press {
                    continue;
                }

                match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) => println!("Нажата клавиша: {}", c),
                    KeyCode::Up => println!("↑"),
                    KeyCode::Down => println!("↓"),
                    KeyCode::Left => println!("←"),
                    KeyCode::Right => println!("→"),
                    _ => {}
                }
            }
        }

        disable_raw_mode().unwrap();
    }
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
