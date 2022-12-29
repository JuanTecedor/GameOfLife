mod cell;
mod engine;
mod game;
mod input_event;

use std::time::{Duration, Instant};

use input_event::InputEvent;

use crate::engine::Engine;
use crate::game::Game;

fn main() {
    let game_size = 128;
    let mut engine = Engine::new();
    let mut game = Game::new_random(game_size);
    engine.draw_game(&game);
    'main_game_loop: loop {
        let t0 = Instant::now();
        for event in engine.handle_events() {
            match event {
                InputEvent::QUIT => {
                    break 'main_game_loop;
                }
                InputEvent::STEP => {
                    engine.update_game(&mut game);
                }
                InputEvent::RESET => {
                    game = Game::new_random(game_size);
                }
                InputEvent::TOGGLE_AUTOSTEP => {}
            }
        }
        if engine.autostep() {
            engine.update_game(&mut game);
        }
        engine.draw_game(&game);
        cap_fps(&t0);
    }
}

fn cap_fps(timepoint: &Instant) {
    const MAX_FPS: f32 = 60.0;
    const MS_PER_FRAME: u128 = (1000.0 / MAX_FPS) as u128;
    let delta = timepoint.elapsed().as_millis();
    if delta < MS_PER_FRAME {
        let remaining_time = MS_PER_FRAME - delta;
        std::thread::sleep(Duration::from_millis(remaining_time as u64));
    }
}
