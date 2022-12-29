use crate::input_event::InputEvent;
use crate::{cell_status::CellStatus, game::Game};

extern crate sdl2;

use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl,
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
            .window(
                "Game of Life | [Q, ESQ] To quit | [R] Reset | [S] Step | [SPACE] Toggle autostep",
                size,
                size,
            )
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
        game.calculate_next_cell_status();
        game.update_cells();
    }

    pub fn draw_game(&mut self, game: &Game) {
        let cell_side_count = game.grid().len();
        let cell_side_size = self.window_size / cell_side_count as u32;

        self.canvas.set_draw_color(Color::WHITE);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::RED);
        for (column_index, column) in game.grid().iter().enumerate() {
            for (cell_index, cell) in column.iter().enumerate() {
                if let CellStatus::ALIVE = cell.current_state() {
                    let x0 = column_index as i32 * cell_side_size as i32;
                    let y0 = cell_index as i32 * cell_side_size as i32;
                    self.canvas
                        .fill_rect(Rect::new(
                            x0 as i32,
                            y0 as i32,
                            cell_side_size,
                            cell_side_size,
                        ))
                        .expect("Could not draw rect");
                }
            }
        }

        self.canvas.set_draw_color(Color::BLACK);
        for i in 0..game.grid().len() {
            let p = i as i32 * cell_side_size as i32;
            self.canvas
                .draw_line((p, 0), (p, self.window_size as i32))
                .expect("Could not draw line");
            self.canvas
                .draw_line((0, p), (self.window_size as i32, p))
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
                    Some(Keycode::R) => {
                        *game = Game::new_random_default_size();
                    }
                    _ => {}
                },
                Event::Quit { .. } => {
                    self.run = false;
                }
                _ => {}
            }
        }
    }

    pub fn run(&self) -> bool {
        self.run
    }

    pub fn autostep(&self) -> bool {
        self.autostep
    }
}
