use raylib::prelude::*;

use crate::{matrix::Matrix, vec2::Vec2, constants};

#[derive(Debug)]
pub struct Game {

    pub width: i32,
    pub height: i32,
    pub title: String,

    pub matrix: Matrix
}

impl Game {
    pub fn new(width: i32, height: i32, title: &str) -> Self {
        let screen_pos:Vec2<i32> = Vec2::new(constants::MATRIX_POS_X, constants::MATRIX_POS_Y);
        let width_height:Vec2<i32> = Vec2::new(constants::MATRIX_WIDTH, constants::MATRIX_HEIGHT);
        Self {
            width,
            height,
            title: title.to_string(),
            matrix: Matrix::new(screen_pos,width_height,constants::MATRIX_CELL_SIZE, constants::MATRIX_PADDING)
        }
    }

    pub fn update(&self) {

    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);
        d.draw_text("Tetris Raylib", 12 ,12, 15, Color::WHITE);
        self.matrix.draw(d);
        self.matrix.draw_border(
            d, 
            self.matrix.screen_pos - (self.matrix.cell_size/2), 
            Vec2::new(self.matrix.width * self.matrix.cell_size, self.matrix.height * self.matrix.cell_size) + (self.matrix.cell_size), 
            self.matrix.cell_size / 2, 
            Color::WHITE);
    }
}
