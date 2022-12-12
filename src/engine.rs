use crate::game::Game;

extern crate sdl2;

use std::cmp;
use sdl2::{
    Sdl, render::Canvas, video::Window,
    pixels::Color, event::Event, keyboard::Keycode, rect::Rect
};

pub struct Engine {
    sdl_context: Sdl,
    window_size: u32,
    canvas: Canvas<Window>,
}

impl Engine {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let size = ({
            let bounds = video_subsystem.display_bounds(0).unwrap();
            cmp::max(bounds.w, bounds.h)
        } as f32 * 0.5) as u32;
        let window = video_subsystem
            .window("Game of Life", size, size)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Self {
            sdl_context,
            window_size : size,
            canvas,
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
                if cell.current_state {
                    let x0 = column_index as i32 * cell_side_size as i32;
                    let y0 = cell_index as i32 * cell_side_size as i32;
                    self.canvas.fill_rect(
                        Rect::new(
                            x0 as i32,
                            y0 as i32,
                            cell_side_size,
                            cell_side_size)
                    ).expect("Could not draw rect");
                }
            }
        }

        self.canvas.set_draw_color(Color::BLACK);
        for i in 0..game.grid().len() {
            let p = i as i32 * cell_side_size as i32;
            self.canvas.draw_line(
                (p, 0),
                (p, self.window_size as i32)
            ).expect("Could not draw line");
            self.canvas.draw_line(
                (0, p),
                (self.window_size as i32, p)
            ).expect("Could not draw line");
        }
        self.canvas.present();
    }

    pub fn exit(&self) -> bool {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } = event {
                return true
            }
        }
        false
    }
}