mod game;
mod map_list;

use crate::game::controller::Controller;
use crate::game::{game::Game, render::Render};
use crate::map_list::MAP_1;

fn main() {
    let mut game = Game {
        map: MAP_1,
        level_index: 0,
    };

    println!("Game is {:?}", game);
    println!("Field is {:?}", game.map.size.get_area());
    println!("map 1 is {:?}", MAP_1);

    let render = Render {};

    render.draw(&game);

    let mut controller = Controller {
        game: &mut game,
        render: &render,
    };

    controller.begin();

    // game.begin();
}
