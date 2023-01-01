use core::panic;

use sdl2::{event::Event, keyboard::Keycode, mouse::MouseButton, Sdl};

use crate::{cell_status::CellStatus, command::Command, engine::Engine, game::Game};

pub fn handle_sdl_events(sdl_context: &Sdl, engine: &Engine, game: &Game) -> Vec<Command> {
    let mut event_pump = sdl_context
        .event_pump()
        .expect("Could not get SDL event pump.");
    let mut commands: Vec<Command> = vec![];
    for event in event_pump.poll_iter() {
        match event {
            Event::KeyUp { .. } => {
                if let Some(command) = handle_key_up(event) {
                    commands.push(command);
                }
            }
            Event::Quit { .. } => {
                commands.push(Command::Quit);
            }
            Event::MouseMotion { .. } | Event::MouseButtonDown { .. } => {
                let cell_side_size = engine.window_size() as usize / game.game_side() as usize;
                if let Some(command) = handle_mouse_event(event, cell_side_size) {
                    commands.push(command);
                }
            }
            _ => {}
        }
    }
    commands
}

fn handle_key_up(event: Event) -> Option<Command> {
    match event {
        Event::KeyUp { keycode, .. } => match keycode {
            Some(Keycode::Escape) | Some(Keycode::Q) => Some(Command::Quit),
            Some(Keycode::Space) => Some(Command::ToggleAutostep),
            Some(Keycode::S) => Some(Command::Step),
            Some(Keycode::Num1) => Some(Command::LoadEmptyGame),
            Some(Keycode::Num2) => Some(Command::LoadRandomGame),
            Some(Keycode::Num3) => Some(Command::LoadExample),
            _ => None,
        },
        _ => {
            panic!("Wrong event type passed to handle key up.");
        }
    }
}

fn handle_mouse_event(event: Event, cell_side_size: usize) -> Option<Command> {
    // TODO
    match event {
        Event::MouseMotion {
            mousestate, x, y, ..
        } => {
            if mousestate.left() {
                Some(handle_click(true, x, y, cell_side_size))
            } else if mousestate.right() {
                Some(handle_click(false, x, y, cell_side_size))
            } else {
                None
            }
        }
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => match mouse_btn {
            MouseButton::Left => Some(handle_click(true, x, y, cell_side_size)),
            MouseButton::Right => Some(handle_click(false, x, y, cell_side_size)),
            _ => None,
        },
        _ => {
            None // TODO
                 // debug_assert!(false, "Wrong event type passed to handle mouse event.");
        }
    }
}

fn handle_click(left_click: bool, screen_x: i32, screen_y: i32, cell_side_size: usize) -> Command {
    // TODO
    Command::SetCellCommand {
        new_status: if left_click {
            CellStatus::ALIVE
        } else {
            CellStatus::DEAD
        },
        grid_x: screen_y as usize / cell_side_size,
        grid_y: screen_x as usize / cell_side_size, // TODO
    }
}
