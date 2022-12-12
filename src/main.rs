mod engine;
mod game;
mod cell;

use std::time::Duration;

use crate::engine::Engine;
use crate::game::Game;

fn main() {
    let mut engine = Engine::new();
    let mut game = Game::new_test_1();
    while !engine.exit() {
        engine.draw_game(&game);
        engine.update_game(&mut game);
        std::thread::sleep(Duration::from_millis(1000));
    }
}
