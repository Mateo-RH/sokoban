use crate::components::{BoxColour, Position};
use crate::entities::*;
use specs::World;

const LEVEL_1: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . R . . . W 
    W . P . . . . W
    W . . . . S . W
    W . . Z . . . W
    W . . . . . . W
    W W W W W W W W
    ";
// N: Nothing
// W: Wall
// P: Player
// B: Blue box
// R: Red box
// S: Red spot
// Z: Blue spot

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
                    b'R' => {
                        create_floor(world, position);
                        create_box(world, position, BoxColour::Red);
                    }
                    b'B' => {
                        create_floor(world, position);
                        create_box(world, position, BoxColour::Blue);
                    }
                    b'S' => {
                        create_floor(world, position);
                        create_box_spot(world, position, BoxColour::Red);
                    }
                    b'Z' => {
                        create_floor(world, position);
                        create_box_spot(world, position, BoxColour::Blue);
                    }
                    b'N' => (),
                    b => panic!("unrecognized map item {}", b),
                }
            });
        });
}
