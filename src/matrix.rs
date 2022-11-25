
use raylib::prelude::*;
use crate::vec2::Vec2;



#[derive(Debug)]
pub struct Matrix {
    pub cells: Vec<Cell>,
    pub screen_pos: Vec2<i32>,
    pub width: i32,
    pub height: i32,
    pub cell_size: i32,
    pub padding: i32
}

impl Matrix {
    pub fn new(screen_pos: Vec2<i32>, width_height: Vec2<i32>, cell_size: i32, padding: i32) -> Self {
        let width = width_height.x;
        let height = width_height.y;
        assert!(width > 0 && height > 0 && cell_size > 0);
        let mut tmp_vec: Vec<Cell> = vec![];
        tmp_vec.resize_with((width * height) as usize, || {Cell::new()});
        Self {
            screen_pos,
            width,
            height, 
            cell_size,
            padding,
            cells: tmp_vec
        }
    }

    pub fn set_cell(&mut self, pos: Vec2<i32>, c: Color) {
        let x = pos.x;
        let y = pos.y;
        assert!(x >= 0 && y >= 0);
        assert!(x < self.width && y < self.height);
        self.cells[(x + y * self.width) as usize].set_color(c);
    }

    pub fn draw_cell(&self, d: &mut RaylibDrawHandle, pos: Vec2<i32>) {
        let x = pos.x;
        let y = pos.y;
        assert!(x >= 0 && x < self.width && y >= 0 && y < self.height);
        let cell_color = self.cells[(x + y * self.width) as usize].get_color();

        let top_left: Vec2<i32> = self.screen_pos + self.padding + (pos * self.cell_size);        
        d.draw_rectangle(
            top_left.x,
            top_left.y,
            self.cell_size - self.padding, 
            self.cell_size - self.padding, 
            cell_color); 
    }

    pub fn draw_border(&self, d: &mut RaylibDrawHandle, pos: Vec2<i32>, width_height: Vec2<i32>, line_thick: i32, color: Color) {
        assert!(pos.x >= 0 && pos.y >= 0 && pos.x < d.get_screen_width() && pos.y < d.get_screen_height());
        assert!(line_thick > 0);
        let rec: Rectangle = Rectangle { x: pos.x as f32, y: pos.y as f32, width: width_height.x as f32, height: width_height.y as f32 };
        d.draw_rectangle_lines_ex(rec, line_thick, color)
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for iy in 0..self.height {
            for ix in 0..self.width {
                let tmp_pos = Vec2::new(ix, iy);
                self.draw_cell(d, tmp_pos);
            }
        }
    }

}

#[derive(Debug)]
pub struct Cell {
    exists: bool,
    color: Color
}

impl Cell {
    pub fn new() -> Self {
        Self {
            exists: true,
            color: Color::GRAY
        }
    }

    pub fn set_color(&mut self, c: Color) {
        self.color = c;
        self.exists = true;
    }
    
    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn remove(&mut self) {
        self.exists = false;
    }
}
