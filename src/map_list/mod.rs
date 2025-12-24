use crate::game::character::Character;
use crate::game::map::{Map, MapStatic};
use crate::game::util::{Position, Size};

pub const MAP_0: MapStatic = MapStatic {
    character: Character {
        position: Position { x: 1, y: 1 },
    },
    point_list: &[Position { x: 4, y: 3 }, Position { x: 5, y: 3 }],
    item_list: &[Position { x: 3, y: 2 }, Position { x: 2, y: 3 }],
    map: "\
#####  \n\
#   #  \n\
# # ###\n\
#     #\n\
#######",
};
