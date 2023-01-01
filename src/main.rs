use engine::Engine;
use fps_limiter::FpsLimiter;

mod cell;
mod cell_status;
mod command;
mod engine;
mod fps_limiter;
mod game;
mod input_handler;
mod level_loader;

fn main() {
    let mut engine = Engine::new();
    let mut fps_limiter = FpsLimiter::new(60.0);

    while engine.run() {
        fps_limiter.start();
        step_game(&mut engine);
        fps_limiter.wait();
    }
}

fn step_game(engine: &mut Engine) {
    engine.handle_events();
    if engine.autostep() {
        engine.update();
    }
    engine.draw_game();
}
