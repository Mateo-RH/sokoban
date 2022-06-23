use std::time::Duration;

use crate::systems::GameplayState;
use ggez::event::KeyCode;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub move_count: u32,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}
