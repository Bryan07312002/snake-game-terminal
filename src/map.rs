use crate::screen::Color;

#[derive(Clone)]
pub struct Cell {
    pub char: char,
    pub color: Color,
}

impl Cell {
    pub fn new(char: char, color: Color) -> Self {
        Self { char, color }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

pub type MapBuffer = Vec<Vec<Cell>>;

pub struct GameMap {
    pub buffer: MapBuffer,
}

impl GameMap {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            buffer: vec![vec![Cell::new(' ', Color::Transtaparent); cols]; rows],
        }
    }

    pub fn center(&self) -> Point {
        let middle_rows = self.buffer.len() / 2;
        let middle_cols = self.buffer[0].len() / 2;

        Point {
            row: middle_rows,
            col: middle_cols,
        }
    }
}
