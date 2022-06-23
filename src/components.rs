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

pub enum RenderableKind {
    Static,
    Animated,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub paths: Vec<String>,
}

impl Renderable {
    pub fn new(paths: Vec<String>) -> Self {
        Self { paths }
    }

    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("Invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }

    pub fn path(&self, path_index: usize) -> String {
        self.paths[path_index % self.paths.len()].clone()
    }
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
