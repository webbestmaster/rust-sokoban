use crate::game::character::Character;
use crate::game::map::Map;
use crate::game::util::{Position};

pub const MAP_1: Map = Map {
    character: Character {
        position: Position { x: 2, y: 1 },
    },
    point_list: &[Position { x: 3, y: 4 }, Position { x: 5, y: 1 }],
    item_list: &[Position { x: 2, y: 3 }, Position { x: 4, y: 6 }],
    map: "\
#############\n\
#   #       #\n\
#   #       #\n\
#   #       #\n\
#           #\n\
#   ####    #\n\
#           #\n\
#############",
};
