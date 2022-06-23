use std::collections::HashMap;
use std::fmt::{self, Display};
use std::time::Duration;

use ggez::graphics::{self, Color, DrawParam, Image};
use ggez::{event::KeyCode, Context};
use glam::Vec2;
use specs::{join::Join, ReadStorage, System, Write, WriteStorage};
use specs::{Entities, Read};

use crate::components::*;
use crate::resources::*;

const TILE_WIDTH: f32 = 32.0;
const MAP_WIDTH: u8 = 8;
const MAP_HEIGHT: u8 = 9;

pub enum GameplayState {
    Playing,
    Won,
}
impl Default for GameplayState {
    fn default() -> Self {
        Self::Playing
    }
}
impl Display for GameplayState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}

// Systems
pub struct InputSystem {}
impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = (
        Write<'a, InputQueue>,
        Write<'a, Gameplay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut gameplay, entities, mut positions, players, movables, immovables) =
            data;

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

        if to_move.len() > 0 {
            gameplay.move_count += 1;
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

pub struct GameplaySystem {}
impl<'a> System<'a> for GameplaySystem {
    type SystemData = (
        Write<'a, Gameplay>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay, boxes, spots, positions) = data;

        let mut blue_in_spot = false;
        let mut red_in_spot = false;

        let blue_boxes = (&boxes, &positions)
            .join()
            .filter(|(b, ..)| match b.colour {
                BoxColour::Blue => true,
                _ => false,
            })
            .map(|(_, p)| (p.x, p.y))
            .collect::<Vec<_>>();

        let red_boxes = (&boxes, &positions)
            .join()
            .filter(|(b, ..)| match b.colour {
                BoxColour::Red => true,
                _ => false,
            })
            .map(|(_, p)| (p.x, p.y))
            .collect::<Vec<_>>();

        for (BoxSpot { colour }, spot) in (&spots, &positions).join() {
            match colour {
                BoxColour::Blue if blue_boxes.contains(&(spot.x, spot.y)) => {
                    blue_in_spot = true;
                }
                BoxColour::Red if red_boxes.contains(&(spot.x, spot.y)) => {
                    red_in_spot = true;
                }
                _ => (),
            }
        }

        if blue_in_spot && red_in_spot {
            gameplay.state = GameplayState::Won;
        }
    }
}

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}
impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, Gameplay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gampelay, time, positions, renderables) = data;
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        for (position, renderable) in rendering_data.iter() {
            let image = self.get_image(renderable, time.delta);
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        self.draw_text(&gampelay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gampelay.move_count.to_string(), 525.0, 100.0);

        graphics::present(self.context).expect("expected to present");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }

    pub fn get_image(&mut self, renderable: &Renderable, delta: Duration) -> Image {
        let path_index = match renderable.kind() {
            RenderableKind::Static => 0,
            RenderableKind::Animated => {
                // If we have multiple, we want to select the right one based on the delta time.
                // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
                // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
                // we technically are on the next iteration of the loop (or on 0), but we will let
                // the renderable handle this logic of wrapping frames.
                ((delta.as_millis() % 1000) / 250) as usize
            }
        };

        let image_path = renderable.path(path_index);

        Image::new(self.context, image_path).expect("expected image")
    }
}
