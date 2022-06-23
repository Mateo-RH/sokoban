use crate::components::*;
use specs::{Builder, World, WorldExt};

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 3, ..position })
        .with(Renderable::new(vec!["/images/wall.png".to_string()]))
        .with(Wall {})
        .with(Immovable)
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 1, ..position })
        .with(Renderable::new(vec!["/images/floor.png".to_string()]))
        .build();
}

pub fn create_box(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 3, ..position })
        .with(Renderable::new(vec![
            format!("/images/box_{}_1.png", colour),
            format!("/images/box_{}_2.png", colour),
        ]))
        .with(Box { colour })
        .with(Movable)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 2, ..position })
        .with(Renderable::new(vec![format!(
            "/images/box_spot_{}.png",
            colour
        )]))
        .with(BoxSpot { colour })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 3, ..position })
        .with(Renderable::new(vec![
            "/images/player_1.png".to_string(),
            "/images/player_2.png".to_string(),
            "/images/player_3.png".to_string(),
        ]))
        .with(Player {})
        .with(Movable)
        .build();
}
