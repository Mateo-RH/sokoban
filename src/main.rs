use ggez::event::{self, KeyCode, KeyMods};
use ggez::{conf, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod entities;
mod map;
mod systems;

use crate::components::register_components;
use crate::map::initialize_level;
use systems::{register_resources, InputQueue, InputSystem, RenderingSystem};

// This struct will hold all our game state
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement two things:
// - updating
// - rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // Run input system
        {
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        // Render game entities
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _context: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}

pub fn main() -> GameResult {
    let mut world = World::new();
    register_components(&mut world);
    register_resources(&mut world);
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(1200.0, 1000.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
