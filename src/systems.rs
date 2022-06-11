use ggez::graphics::{self, DrawParam, Image};
use ggez::{event::KeyCode, Context};
use glam::Vec2;
use specs::{join::Join, ReadStorage, System, World, Write, WriteStorage};

use crate::components::{Player, Position, Renderable};

const TILE_WIDTH: f32 = 32.0;

// TODO: explain Resources
#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

// Registering resources
pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

// Systems
pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    // Data
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, players) = data;

        for (position, _player) in (&mut positions, &players).join() {
            // Get the first key pressed
            if let Some(key) = input_queue.keys_pressed.pop() {
                // Apply the key to the position
                match key {
                    KeyCode::Up => position.y -= 1,
                    KeyCode::Down => position.y += 1,
                    KeyCode::Left => position.x -= 1,
                    KeyCode::Right => position.x += 1,
                    _ => (),
                }
            }
        }
    }
}

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}
// System implementation
impl<'a> System<'a> for RenderingSystem<'a> {
    // Data
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, (positions, renderables): Self::SystemData) {
        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (position, renderable) in rendering_data.iter() {
            // Load the image
            let image = Image::new(self.context, &renderable.path).expect("expected image");
            let x = position.x as f32 * TILE_WIDTH;
            let y = position.y as f32 * TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(Vec2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything
        // on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
