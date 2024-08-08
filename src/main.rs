mod map;
mod screen;
mod snake;

use map::{GameMap, Point};
use rand::Rng;
use screen::Screen;
use snake::{Direction, Snake};
use std::{
    io::{self, stdin, stdout, Write},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen::IntoAlternateScreen,
    terminal_size,
};

pub enum GameState {
    GameOver,
    InGame,
}

struct Game {
    pub map: GameMap,
    pub snake: Snake,
    pub screen: Screen,
    pub should_end: bool,
    pub apple: Point,
    pub state: GameState,
    pub initial_snake_size: usize,
}

impl Game {
    pub fn new() -> Self {
        let (cols, rows) = terminal_size().unwrap();
        // FIXME: at least understand why this is a bug, the screen looks smaller than its said
        let screen = Screen::new(rows - 2, cols - 1);
        let map = GameMap::new(rows as usize, cols as usize);

        let point = map.center();
        let snake = Snake::new(Point {
            col: point.col,
            row: point.row,
        });

        let mut rng = rand::thread_rng();
        let rand_col = rng.gen_range(0..screen.cols);
        let rand_row = rng.gen_range(0..screen.rows);

        let apple = Point {
            col: rand_col as usize,
            row: rand_row as usize,
        };

        return Self {
            map,
            screen,
            snake,
            should_end: false,
            apple,
            state: GameState::InGame,
            initial_snake_size: 3,
        };
    }

    fn generate_new_apple(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_col = rng.gen_range(0..self.screen.cols / 2);
        let rand_row = rng.gen_range(0..self.screen.rows / 2);

        self.apple = Point {
            col: rand_col as usize,
            row: rand_row as usize,
        };
    }

    pub fn check_wall_imits(&mut self) {
        if &self.snake.body[0].col == &usize::from(self.screen.cols) {
            match self.snake.direction {
                Direction::Right => self.snake.body[0].col = 0,
                _ => {}
            }
        }

        if &self.snake.body[0].col == &0 {
            match self.snake.direction {
                Direction::Left => self.snake.body[0].col = self.screen.cols.clone() as usize,
                _ => {}
            }
        }

        if &self.snake.body[0].row == &usize::from(self.screen.rows) {
            match self.snake.direction {
                Direction::Down => self.snake.body[0].row = 0,
                _ => {}
            }
        }

        if &self.snake.body[0].row == &0 {
            match self.snake.direction {
                Direction::Up => self.snake.body[0].row = self.screen.rows.clone() as usize,
                _ => {}
            }
        }
    }

    fn update_snake(&mut self) {
        // TODO: Check if snake eats apple
        if self.snake.body[0] == self.apple {
            self.snake.add_body_piece();
            self.generate_new_apple();
        }

        let old_snake = self.snake.clone();

        self.check_wall_imits();
        self.snake.goto_next_pos();

        // clear the old snake positions
        let diff: Vec<Point> = old_snake
            .body
            .iter()
            .filter(|&p| !self.snake.body.contains(p))
            .cloned()
            .collect();

        for point in &diff {
            self.screen.clear_cell(point);
        }

        // TODO: Check if snake bites own tail
        if self.snake.is_biteing_own_tail() {
            self.state = GameState::GameOver;
        }
    }

    fn restart_game(&mut self) {
        let point = self.map.center();
        self.snake = Snake::new(Point {
            col: point.col,
            row: point.row,
        });

        let mut rng = rand::thread_rng();
        let rand_col = rng.gen_range(0..self.screen.cols);
        let rand_row = rng.gen_range(0..self.screen.rows);
        self.apple = Point {
            col: rand_col as usize,
            row: rand_row as usize,
        };

        self.screen.print_map(&self.map.buffer);
    }

    fn print_score(&self) {
        print!(
            "{}{}",
            termion::cursor::Goto(1, 1),
            self.snake.body.len() - self.initial_snake_size
        );
    }

    fn update_in_game_tick(&mut self) {
        self.update_snake();

        self.print_score();
        self.screen.print_snake(&self.snake);
        self.screen.print_apple(self.apple);
    }

    fn update_tick(&mut self) {
        match self.state {
            GameState::GameOver => self.screen.print_game_over(self.map.center()),
            GameState::InGame => self.update_in_game_tick(),
        }
    }
}

fn listen_key_events(game: &Arc<Mutex<Game>>, stdout: &mut RawTerminal<io::Stdout>) {
    let stdin = stdin();

    for c in stdin.keys() {
        let mut game = game.lock().unwrap();

        match c.unwrap() {
            Key::Ctrl('q') => {
                println!("should end");
                game.should_end = true;
            }
            Key::Left => {
                if let Direction::Right = game.snake.direction {
                    return;
                }

                game.snake.direction = Direction::Left;
            }
            Key::Right => {
                if let Direction::Left = game.snake.direction {
                    return;
                }

                game.snake.direction = Direction::Right;
            }
            Key::Up => {
                if let Direction::Down = game.snake.direction {
                    return;
                }

                game.snake.direction = Direction::Up;
            }
            Key::Down => {
                if let Direction::Up = game.snake.direction {
                    return;
                }

                game.snake.direction = Direction::Down;
            }
            Key::Char('r') => game.restart_game(),
            Key::Char('d') => game.state = GameState::GameOver,
            _ => {}
        }

        stdout.flush().unwrap();
        break;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let game = Arc::new(Mutex::new(Game::new()));
    let game_clone_for_keys = Arc::clone(&game);

    // change screen so main screen dont get all messed up
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();

    let tick_time = Duration::from_millis(200);

    thread::spawn(move || loop {
        {
            listen_key_events(&game_clone_for_keys, &mut screen);

            let game = game_clone_for_keys.lock().unwrap();
            if game.should_end {
                break;
            }
        }

        thread::sleep(Duration::from_millis(100));
    });

    print!("{}", termion::cursor::Hide);
    {
        let game = game.lock().unwrap();
        game.screen.print_map(&game.map.buffer);
    }
    loop {
        {
            let mut game = game.lock().unwrap();

            if game.should_end {
                break;
            }

            game.update_tick();
        }

        // Sleep outside the lock to minimize the time the lock is held
        thread::sleep(tick_time);
    }

    print!("{}", termion::cursor::Show);

    Ok(())
}
