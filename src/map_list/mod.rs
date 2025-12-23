use crate::game::map::Map;
use crate::game::util::{Position, Size};

pub const MAP_1: Map = Map {
    size: Size {
        height: 20,
        width: 20,
    },
    start_position: Position { x: 3, y: 4 },
    win_point_list: &[Position { x: 3, y: 4 }, Position { x: 5, y: 1 }],
    data: "\
#############\
#   #   0   #\
#   #   0   #\
#   # @     #\
#           #\
#   ####    #\
#  +     +  #\
#############",
};
