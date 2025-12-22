mod game;
mod map_list;

use crate::game::{game::Game, map::Map, util::Size};
use crate::map_list::{map_1};

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
    println!("map 1 is {:?}", map_1);

    // game.begin();
}
