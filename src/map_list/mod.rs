use crate::game::character::Character;
use crate::game::map::Map;
use crate::game::util::{Position, Size};

pub const MAP_1: Map = Map {
    size: Size {
        height: 10,
        width: 14,
    },
    character: Character {
        position: Position { x: 2, y: 1 },
    },
    point_list: &[Position { x: 3, y: 4 }, Position { x: 5, y: 1 }],
    item_list: &[Position { x: 3, y: 4 }, Position { x: 5, y: 1 }],
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
