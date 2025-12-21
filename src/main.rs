mod game;

use game::{Game, Map, Size};

fn main() {
    let game = Game {
        field: Size {
            height: 12,
            width: 23,
        },
        level_index: 0,
    };

    let map = Map {
        size: Size {
            width: 12,
            height: 12,
        },
        data: String::from("###\n # \n ##"),
    };

    println!("Game is {:?}", game);
    println!("Map is {}", map.data);
    println!("Field is {:?}", game.field.get_area());

    game.begin();
}
