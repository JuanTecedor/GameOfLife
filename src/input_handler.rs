use core::panic;

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, Sdl};

use crate::{cell_status::CellStatus, engine::Engine, game::Game, command::Command};

pub fn handle_sdl_event(sdl_context: Sdl) -> Vec<Command> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not get SDL event pump.");
    let mut commands: Vec<Command>= vec![];
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyUp { .. } => {
                if let Some(command) = handle_key_up(event) {
                    commands.push(command);
                }
            }
            Event::Quit { .. } => {
                commands.push(Command::Quit);
                return commands;
            }
            Event::MouseMotion { .. } | Event::MouseButtonDown { .. } => {
                // TODO
                handle_mouse_event(self, game, event);
            }
            _ => { None }
        }
    }
    commands
}

fn handle_key_up(event: Event) -> Option<Command> {
    match event {
        Event::KeyUp { keycode, .. } => match keycode {
            Some(Keycode::Escape) | Some(Keycode::Q) => {
                Some(Command::Quit)
            }
            Some(Keycode::Space) => {
                Some(Command::ToggleAutostep)
            }
            Some(Keycode::S) => {
                Some(Command::Step)
            }
            Some(Keycode::Num1) => {
                Some(Command::LoadEmptyGame)
            }
            Some(Keycode::Num2) => {
                Some(Command::LoadRandomGame)
            }
            Some(Keycode::Num3) => {
                Some(Command::LoadExample)
            }
            _ => { None }
        },
        _ => {
            panic!("Wrong event type passed to handle key up.");
        }
    }
}

fn handle_mouse_event(engine: &Engine, game: &mut Game, event: Event) {
    // TODO
    match event {
        Event::MouseMotion {
            mousestate, x, y, ..
        } => {
            if mousestate.left() {
                handle_click(engine, game, true, x, y);
            } else if mousestate.right() {
                handle_click(engine, game, false, x, y);
            }
        }
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => match mouse_btn {
            MouseButton::Left => {
                handle_click(engine, game, true, x, y);
            }
            MouseButton::Right => {
                handle_click(engine, game, false, x, y);
            }
            _ => {}
        },
        _ => {
            debug_assert!(false, "Wrong event type passed to handle mouse event.");
        }
    }
}

fn handle_click(engine: &Engine, game: &mut Game, left_click: bool, x: i32, y: i32) -> Command {
    // TODO
    Command::SetCellCommand{
        new_status: if left_click {
            CellStatus::ALIVE
        } else {
            CellStatus::DEAD
        }
    }
    let cell_side_size = engine.window_size() as usize / game.game_side() as usize;
    game.set_current_state(
        y as usize / cell_side_size,
        x as usize / cell_side_size,
        if left_click {
            CellStatus::ALIVE
        } else {
            CellStatus::DEAD
        },
    );
}
