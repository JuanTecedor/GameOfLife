mod cell;
mod cell_status;
mod engine;
mod fps_limiter;
mod game;
mod level_loader;

use fps_limiter::FpsLimiter;

use crate::engine::Engine;
use crate::game::Game;

fn main() {
    let mut engine = Engine::new();
    let mut game = Game::new_random_default_size();
    let mut fps_limiter = FpsLimiter::new(60.0);

    while engine.run() {
        fps_limiter.start();
        step_game(&mut engine, &mut game);
        fps_limiter.wait();
    }
}

fn step_game(engine: &mut Engine, game: &mut Game) {
    engine.handle_events(game);
    if engine.autostep() {
        engine.update_game(game);
    }
    engine.draw_game(game);
}
