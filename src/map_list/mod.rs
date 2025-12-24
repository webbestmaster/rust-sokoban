use crate::game::character::Character;
use crate::game::map::{Map, MapStatic};
use crate::game::util::{Position, Size};

pub const MAP_1: MapStatic = MapStatic {
    character: Character {
        position: Position { x: 2, y: 1 },
    },
    point_list: &[Position { x: 3, y: 4 }, Position { x: 5, y: 1 }],
    item_list: &[Position { x: 2, y: 1 }, Position { x: 3, y: 6 }],
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
