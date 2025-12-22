use crate::game::game::Game;

pub struct Render {
    
}

impl Render {
    pub fn draw(&self, game: &Game) {
        let state = game.get_state();
        
        println!("{:?}", state);
    }
}