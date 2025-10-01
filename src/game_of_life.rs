use rand::prelude::*;

pub const PIXEL_WIDTH: u8 = 2;
pub const TRAIL_LIFETIME_MAX: u8 = 100;
const TRAIL_LIFETIME_DECREASE: u8 = 5;

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
    pub neighbors: i8,
    pub trail_lifetime: u8,
}

pub struct GameOfLife {
    board: Vec<Vec<Cell>>,
    step: u128,
}

fn update_neighbors_count(board: &mut Vec<Vec<Cell>>, x: usize, y: usize) {
    let change = if board[y][x].alive { 1 } else { -1 };

    let t = y.checked_sub(1).unwrap_or(board.len() - 1);
    let l = x.checked_sub(1).unwrap_or(board[0].len() - 1);
    let r = if x + 1 >= board[0].len() { 0 } else { x + 1 };
    let b = if y + 1 >= board.len() { 0 } else { y + 1 };

    board[t][x].neighbors += change;
    board[t][l].neighbors += change;
    board[t][r].neighbors += change;

    board[y][l].neighbors += change;
    board[y][r].neighbors += change;

    board[b][x].neighbors += change;
    board[b][l].neighbors += change;
    board[b][r].neighbors += change;
}

fn set_cell(board: &mut Vec<Vec<Cell>>, x: usize, y: usize, alive: bool) {
    if alive == board[y][x].alive {
        return;
    }

    board[y][x].alive = alive;
    if alive {
        board[y][x].trail_lifetime = TRAIL_LIFETIME_MAX;
    }
    update_neighbors_count(board, x, y);
}

impl GameOfLife {
    pub fn new() -> Self {
        let mut game = Self {
            board: vec![],
            step: 0,
        };
        game.create_random_board();
        game
    }

    pub fn create_random_board(&mut self) {
        let term_size = termion::terminal_size().unwrap();
        let rows = term_size.1 - 1;
        let columns = term_size.0 / PIXEL_WIDTH as u16;

        let mut rng = rand::rng();
        self.board = vec![];
        self.step = 0;

        for _ in 0..rows {
            self.board.push(
                (0..columns)
                    .map(|_| {
                        let alive = rng.random_bool(0.4);
                        Cell {
                            alive: alive,
                            neighbors: 0,
                            trail_lifetime: if alive { TRAIL_LIFETIME_MAX } else { 0 },
                        }
                    })
                    .collect(),
            );
        }
        self.init_neighbors_count();
    }

    pub fn clear_board(&mut self) {
        for row in self.board.iter_mut() {
            for cell in row {
                cell.alive = false;
                cell.neighbors = 0;
                cell.trail_lifetime = 0;
            }
        }
    }

    fn init_neighbors_count(&mut self) {
        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                let mut neighbors = 0;

                let t = y.checked_sub(1).unwrap_or(self.board.len() - 1);
                let l = x.checked_sub(1).unwrap_or(self.board[0].len() - 1);
                let r = if x + 1 >= self.board[0].len() {
                    0
                } else {
                    x + 1
                };
                let b = if y + 1 >= self.board.len() { 0 } else { y + 1 };

                neighbors += if self.board[t][x].alive { 1 } else { 0 };
                neighbors += if self.board[t][l].alive { 1 } else { 0 };
                neighbors += if self.board[t][r].alive { 1 } else { 0 };

                neighbors += if self.board[y][l].alive { 1 } else { 0 };
                neighbors += if self.board[y][r].alive { 1 } else { 0 };

                neighbors += if self.board[b][x].alive { 1 } else { 0 };
                neighbors += if self.board[b][l].alive { 1 } else { 0 };
                neighbors += if self.board[b][r].alive { 1 } else { 0 };

                self.board[y][x].neighbors = neighbors;
            }
        }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        let toggle_value = !self.board[y][x].alive;
        set_cell(&mut self.board, x, y, toggle_value);
    }

    pub fn update(&mut self) {
        self.step += 1;
        let mut back_buffer = self.board.clone();

        for y in 0..self.board.len() {
            for x in 0..self.board[0].len() {
                match self.board[y][x].neighbors {
                    2 => {}                                       // keep alive (nothing to do)
                    3 => set_cell(&mut back_buffer, x, y, true),  // come to life or stay alive
                    _ => set_cell(&mut back_buffer, x, y, false), // die
                }

                if back_buffer[y][x].trail_lifetime > 0 && !back_buffer[y][x].alive {
                    back_buffer[y][x].trail_lifetime = back_buffer[y][x]
                        .trail_lifetime
                        .checked_sub(TRAIL_LIFETIME_DECREASE)
                        .unwrap_or(0);
                }
            }
        }

        self.board = back_buffer;
    }

    pub fn board(&self) -> &Vec<Vec<Cell>> {
        &self.board
    }

    pub fn step(&self) -> &u128 {
        &self.step
    }
}
