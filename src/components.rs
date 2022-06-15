use std::fmt::{Display, Formatter, Result};

use specs::{Component, NullStorage, VecStorage};

pub enum BoxColour {
    Red,
    Blue,
}
impl Display for BoxColour {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        });
        Ok(())
    }
}

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub colour: BoxColour,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub colour: BoxColour,
}
