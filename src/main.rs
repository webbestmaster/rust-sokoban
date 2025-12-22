mod game;

use crate::game::{game::Game, map::Map, util::Size};

fn main() {
    let game = Game {
        map: Map {
            size: Size {
                width: 20,
                height: 20,
            },
            data: "###\n # \n ##",
        },
        level_index: 0,
    };

    println!("Game is {:?}", game);
    println!("Field is {:?}", game.map.size.get_area());

    // game.begin();
}
