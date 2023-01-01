use crate::{
    cell_status::CellStatus,
    game::Game, input_handler::handle_sdl_events, command::Command,
};

extern crate sdl2;

use sdl2::{event::Event, pixels::Color, rect::Rect, render::Canvas, video::Window, Sdl};
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

    pub fn handle_events(&self, game: &Game) {
        let commands = handle_sdl_events(&self.sdl_context, self, game);
        for command in commands {
            match command {
                Command::Quit => {

                }
                Command::Step => {

                }
                Command::ToggleAutostep => {

                }
                Command::LoadRandomGame => {

                }
                Command::LoadEmptyGame => {

                }
                Command::LoadExample => {

                }
                Command::SetCellCommand{ new_status, grid_x, grid_y } => {

                }
            }
        }
    }

    pub fn window_size(&self) -> u32 {
        self.window_size
    }

    pub fn run(&self) -> bool {
        self.run
    }

    pub fn autostep(&self) -> bool {
        self.autostep
    }

    pub fn toggle_autostep(&mut self) {
        self.autostep ^= true;
    }

    pub fn stop(&mut self) {
        self.run = false;
    }
}
