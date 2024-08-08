use crate::map::Point;

#[derive(Clone)]
pub struct Snake {
    pub body: Vec<Point>,
    pub direction: Direction,
}

impl Snake {
    pub fn new(head_pos: Point) -> Self {
        Self {
            body: vec![
                Point {
                    col: head_pos.col,
                    row: head_pos.row,
                },
                Point {
                    col: head_pos.col,
                    row: head_pos.row + 1,
                },
                Point {
                    col: head_pos.col,
                    row: head_pos.row + 2,
                },
            ],
            direction: Direction::Up,
        }
    }

    pub fn goto_next_pos(&mut self) {
        let mut last_pos = self.body[0].clone();
        match &self.direction {
            Direction::Up => {
                self.body[0].row = self.body[0].row - 1;
            }
            Direction::Down => {
                self.body[0].row = self.body[0].row + 1;
            }
            Direction::Left => {
                self.body[0].col = self.body[0].col - 1;
            }
            Direction::Right => {
                self.body[0].col = self.body[0].col + 1;
            }
        }

        // make the rest of the body move
        for i in 1..self.body.len() {
            let tmp = self.body[i].clone();
            self.body[i] = last_pos;
            last_pos = tmp;
        }
    }

    pub fn add_body_piece(&mut self) {
        let mut last_pos = self.body.last().unwrap().clone();
        last_pos.row = last_pos.row + 1;
        self.body.push(last_pos);
    }

    pub fn is_biteing_own_tail(&self) -> bool {
        let head = self.head().to_owned();

        for i in 1..self.body.len() {
            if self.body[i] == head {
                return true;
            }
        }

        return false;
    }

    pub fn head(&self) -> &Point {
        &self.body[0]
    }
}

#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
