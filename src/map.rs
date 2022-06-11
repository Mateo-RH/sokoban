use crate::components::Position;
use crate::entities::*;
use specs::World;

const LEVEL_1: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W 
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

pub fn initialize_level(world: &mut World) {
    LEVEL_1
        .to_string()
        .trim()
        .split('\n')
        .map(|x| x.replace(" ", ""))
        .enumerate()
        .for_each(|(y, row)| {
            row.as_bytes().iter().enumerate().for_each(|(x, byte)| {
                let position = Position {
                    x: x as u8,
                    y: y as u8,
                    z: 0,
                };
                match *byte {
                    b'.' => create_floor(world, position),
                    b'W' => {
                        create_floor(world, position);
                        create_wall(world, position);
                    }
                    b'P' => {
                        create_floor(world, position);
                        create_player(world, position);
                    }
                    b'B' => {
                        create_floor(world, position);
                        create_box(world, position);
                    }
                    b'S' => {
                        create_floor(world, position);
                        create_box_spot(world, position);
                    }
                    b'N' => (),
                    b => panic!("unrecognized map item {}", b),
                }
            });
        });
}
