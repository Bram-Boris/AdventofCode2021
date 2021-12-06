use core::panic;

use crate::utils;

#[derive(Clone, PartialEq)]
struct Board {
    values: Vec<Vec<(u32, bool)>>,
}

struct Bingo {
    boards: Vec<Board>,
    won_boards: Vec<Board>,
    order: Vec<u32>,
    total_boards: usize,
}

impl Bingo {
    fn new() -> Bingo {
        let data = utils::read_lines_to_string("inputs/day4.txt");
        let order = data
            .lines()
            .next()
            .expect("No input")
            .split(",")
            .map(|char| char.parse().expect("Is a number!"))
            .collect::<Vec<u32>>();

        let mut boards = Vec::<Board>::new();
        let lines = data.lines().skip(2);

        let mut board = Board::new();

        for next in lines {
            if next == "" {
                boards.push(board.clone());
                board = Board::new();
            } else {
                board.values.push(
                    next.split_whitespace()
                        .map(|char| (char.parse().expect("Is a number!"), false))
                        .collect::<Vec<(u32, bool)>>(),
                );
            }
        }

        boards.push(board);

        let len = boards.len();

        Bingo {
            boards,
            won_boards: Vec::<Board>::new(),
            order,
            total_boards: len,
        }
    }
    fn draw_number(&mut self, number: u32) {
        self.boards.iter_mut().for_each(|b| {
            b.mark(number);
            if b.won() {
                self.won_boards.push(b.clone());
            }
        });

        self.boards.retain(|b| !b.won());
    }
}
impl Board {
    fn new() -> Board {
        Board {
            values: Vec::<Vec<(u32, bool)>>::new(),
        }
    }

    fn mark(&mut self, int: u32) {
        let flat = self
            .values
            .iter()
            .flat_map(|v| v.iter().map(|item| item.0))
            .collect::<Vec<u32>>();
        match flat.iter().position(|i| *i == int) {
            Some(value) => {
                let index = value % 5;
                let row = value / 5;

                self.values[row][index].1 = true;
            }
            None => {}
        }
    }

    fn won(&self) -> bool {
        let mut horizontal = false;
        let mut vertical = false;
        for row in &self.values {
            if row.iter().all(|tuple| tuple.1 == true) {
                horizontal = true;
            }
        }

        for column in utils::transpose(self.values.clone()) {
            if column.iter().all(|tuple| tuple.1 == true) {
                vertical = true;
            }
        }

        horizontal || vertical
    }

    fn earnings(&self) -> u32 {
        self.values
            .iter()
            .flat_map(|v| {
                v.iter().filter_map(|item| match item.1 {
                    false => Some(item.0),
                    true => None,
                })
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>()
    }
}

pub fn solve() -> u32 {
    let mut bingo = Bingo::new();
    for number in bingo.order.clone() {
        bingo.draw_number(number);

        if let Some(value) = bingo.won_boards.first() {
            return value.earnings() * number;
        }
    }

    panic!("No solution found!");
}

pub fn solve_last_board() -> u32 {
    let mut bingo = Bingo::new();
    for number in bingo.order.clone() {
        bingo.draw_number(number);
        if bingo.total_boards == bingo.won_boards.len() {
            return bingo.won_boards.last().unwrap().earnings() * number;
        }
    }

    panic!("No solution found!");
}

pub fn solve_first_puzzle() {
    println!("D4P1: {}", solve())
}

pub fn solve_second_puzzle() {
    println!("D4P2: {}", solve_last_board());
}
