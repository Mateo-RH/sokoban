use std::collections::HashMap;

use ggez::graphics::{self, DrawParam, Image};
use ggez::{event::KeyCode, Context};
use glam::Vec2;
use specs::Entities;
use specs::{join::Join, ReadStorage, System, Write, WriteStorage};

use crate::components::{Immovable, Movable, Player, Position, Renderable};

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

// Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

// Systems
pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = (
        Write<'a, InputQueue>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, entities, mut positions, players, movables, immovables) = data;

        let mut to_move = Vec::new();

        for (position, ..) in (&positions, &players).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                let mov = (&entities, &movables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                let (start, end, is_x) = match key {
                    KeyCode::K => (position.y, 0, false),
                    KeyCode::J => (position.y, MAP_HEIGHT, false),
                    KeyCode::H => (position.x, 0, true),
                    KeyCode::L => (position.x, MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => match immov.get(&pos) {
                            Some(_) => to_move.clear(),
                            None => break,
                        },
                    }
                }
            }
        }

        // Now actually move what needs to be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(position) = position {
                match key {
                    KeyCode::K => position.y -= 1,
                    KeyCode::J => position.y += 1,
                    KeyCode::H => position.x -= 1,
                    KeyCode::L => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (positions, renderables): Self::SystemData) {
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let image = Image::new(self.context, &renderable.path).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        graphics::present(self.context).expect("expected to present");
    }
}
