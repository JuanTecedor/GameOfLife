use crate::{cell_status::CellStatus, game::Game, level_loader::load_level};

extern crate sdl2;

use sdl2::{
    event::Event, keyboard::Keycode, mouse::MouseButton, pixels::Color, rect::Rect, render::Canvas,
    video::Window, Sdl,
};
use std::cmp;

pub struct Engine {
    sdl_context: Sdl,
    window_size: u32,
    canvas: Canvas<Window>,
    run: bool,
    autostep: bool,
}

impl Engine {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().expect("Could not init SDL2.");
        let video_subsystem = sdl_context
            .video()
            .expect("Could not get SDL video context.");
        let size = ({
            let bounds = video_subsystem
                .display_bounds(0)
                .expect("Could not get SDL display bounds.");
            cmp::max(bounds.w, bounds.h)
        } as f32
            * 0.6) as u32;
        let window = video_subsystem
            .window("Game of Life", size, size)
            .position_centered()
            .build()
            .expect("Could not build SDL window.");
        let canvas = window
            .into_canvas()
            .build()
            .expect("Could not get SDL window canvas.");
        Self {
            sdl_context,
            window_size: size,
            canvas,
            run: true,
            autostep: false,
        }
    }

    pub fn update_game(&self, game: &mut Game) {
        game.update_cells();
    }

    pub fn draw_game(&mut self, game: &Game) {
        let cell_side_count = game.game_side();
        let cell_side_size = self.window_size as f32 / cell_side_count as f32;

        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RED);
        for (column_index, column) in game.grid().iter().enumerate() {
            for (cell_index, cell) in column.iter().enumerate() {
                if let CellStatus::ALIVE = cell.current_state() {
                    let x0 = cell_index as f32 * cell_side_size;
                    let y0 = column_index as f32 * cell_side_size;
                    self.canvas
                        .fill_rect(Rect::new(
                            x0 as i32,
                            y0 as i32,
                            cell_side_size as u32
                                + if cell_index + 1 == cell_side_count {
                                    1
                                } else {
                                    0
                                },
                            cell_side_size as u32
                                + if column_index + 1 == cell_side_count {
                                    1
                                } else {
                                    0
                                },
                        ))
                        .expect("Could not draw rect");
                }
            }
        }

        self.canvas.set_draw_color(Color::BLACK);
        for i in 0..game.grid().len() {
            let p = i as f32 * cell_side_size;
            self.canvas
                .draw_line((p as i32, 0), (p as i32, self.window_size as i32))
                .expect("Could not draw line");
            self.canvas
                .draw_line((0, p as i32), (self.window_size as i32, p as i32))
                .expect("Could not draw line");
        }
        self.canvas.present();
    }

    pub fn handle_events(&mut self, game: &mut Game) {
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .expect("Could not get SDL event pump.");
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::Escape) | Some(Keycode::Q) => {
                        self.run = false;
                    }
                    Some(Keycode::Space) => {
                        self.autostep ^= true;
                    }
                    Some(Keycode::S) => {
                        self.update_game(game);
                    }
                    Some(Keycode::Num1) => {
                        *game = Game::new_empty_default_size();
                    }
                    Some(Keycode::Num2) => {
                        *game = Game::new_random_default_size();
                    }
                    Some(Keycode::Num3) => {
                        if let Ok(level) = load_level() {
                            *game = Game::new(level);
                        }
                    }
                    _ => {}
                },
                Event::Quit { .. } => {
                    self.run = false;
                }
                Event::MouseMotion {
                    mousestate, x, y, ..
                } => {
                    if mousestate.left() {
                        self.handle_click(true, game, x, y);
                    } else if mousestate.right() {
                        self.handle_click(false, game, x, y);
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => {
                        self.handle_click(true, game, x, y);
                    }
                    MouseButton::Right => {
                        self.handle_click(false, game, x, y);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    fn handle_click(&self, left_click: bool, game: &mut Game, x: i32, y: i32) {
        let cell_side_size = self.window_size as usize / game.game_side() as usize;
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

    pub fn run(&self) -> bool {
        self.run
    }

    pub fn autostep(&self) -> bool {
        self.autostep
    }
}
