mod engine;
mod game;
mod cell;

use std::time::{Duration, Instant};

use crate::engine::Engine;
use crate::game::Game;

fn main() {
    let mut engine = Engine::new();
    let mut game = Game::new_random(128);
    while !engine.exit() {
        let t0 = Instant::now();
        engine.draw_game(&game);
        engine.update_game(&mut game);
        cap_fps(&t0);
    }
}

fn cap_fps(timepoint: &Instant) {
    const MAX_FPS : f32 = 5.0;
    const MS_PER_FRAME : u128 = (1000.0 / MAX_FPS) as u128;
    let delta = timepoint.elapsed().as_millis();
    if delta < MS_PER_FRAME {
        let remaining_time = MS_PER_FRAME - delta;
        std::thread::sleep(Duration::from_millis(remaining_time as u64));
    }
}
