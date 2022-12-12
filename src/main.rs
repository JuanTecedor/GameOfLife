mod engine;
mod game;

use crate::engine::Engine;
use crate::game::Game;

fn main() {
    let mut engine = Engine::new();
    let game = Game::new_test_1();
    while let false = engine.exit() {
        engine.draw_game(&game);
    }
}
