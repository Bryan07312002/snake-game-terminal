use crate::{
    map::{MapBuffer, Point},
    snake::Snake,
};

#[derive(Clone)]
pub enum Color {
    Red,
    Transtaparent,
}

pub struct Screen {
    pub cols: u16,
    pub rows: u16,
}

impl Screen {
    pub fn new(rows: u16, cols: u16) -> Self {
        Self { cols, rows }
    }

    pub fn print_map(&self, buf: &MapBuffer) {
        for (row_index, row) in buf.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                println!(
                    "{}{}",
                    // Goto is "one-based", it starts at one not zero
                    termion::cursor::Goto((col_index + 1) as u16, (row_index + 1) as u16),
                    cell.char,
                );
            }
        }
    }

    pub fn print_snake(&self, snake: &Snake) {
        let head_char = 'o';

        println!(
            "{}{}#",
            // Goto is "one-based", it starts at one not zero
            termion::cursor::Goto(
                (snake.body[0].col + 1) as u16,
                (snake.body[0].row + 1) as u16
            ),
            termion::color::Fg(termion::color::White),
        );

        for i in 1..snake.body.len() {
            let body_piece = snake.body[i];
            println!(
                "{}{}#",
                // Goto is "one-based", it starts at one not zero
                termion::cursor::Goto((body_piece.col + 1) as u16, (body_piece.row + 1) as u16),
                termion::color::Fg(termion::color::White),
            );
        }
    }

    pub fn print_apple(&self, pos: Point) {
        println!(
            "{}{}*",
            // Goto is "one-based", it starts at one not zero
            termion::cursor::Goto((pos.col + 1) as u16, (pos.row + 1) as u16),
            termion::color::Fg(termion::color::Red),
        );
    }

    pub fn print_game_over(&self, center: Point) {
        let message = "GAME OVER";
        println!(
            "{}{message}",
            termion::cursor::Goto(
                ((center.col - message.len()) - 1) as u16,
                (center.row) as u16
            ),
        )
    }

    pub fn clear_cell(&self, pos: &Point) {
        println!(
            "{}{} ",
            // Goto is "one-based", it starts at one not zero
            termion::cursor::Goto((pos.col + 1) as u16, (pos.row + 1) as u16),
            termion::color::Fg(termion::color::Reset),
        );
    }
}
