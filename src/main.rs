mod game;
mod map_list;

use crate::game::controller::Controller;
use crate::game::{game::Game, render::Render};
use crate::game::map::Map;
use crate::map_list::MAP_0;

fn main() {
    let mut game = Game {
        map: Map::to_map(MAP_0),
        level_index: 0,
    };

    println!("Game is {:?}", game);
    println!("Field is {:?}", game.map.get_size());
    // println!("map 1 is {:?}", MAP_1);

    let render = Render {};

    render.draw(&game);

    let mut controller = Controller {
        game: &mut game,
        render: &render,
    };

    controller.begin();

    // game.begin();
}
