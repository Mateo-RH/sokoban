use ggez::event::{self, KeyCode, KeyMods};
use ggez::{conf, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;

mod components;
mod entities;
mod map;
mod systems;

use crate::components::*;
use crate::map::initialize_level;
use systems::*;

struct Game {
    world: World,
}

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        let mut is = InputSystem {};
        is.run_now(&self.world);

        //TODO: test dispatcher?
        let mut gs = GameplaySystem {};
        gs.run_now(&self.world);

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mut rs = RenderingSystem { context };
        rs.run_now(&self.world);
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

    //Components
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
    //Resources
    world.insert(InputQueue::default());
    world.insert(Gameplay::default());
    world.insert(GameplayState::default());

    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(1200.0, 1000.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;

    event::run(context, event_loop, Game { world })
}
