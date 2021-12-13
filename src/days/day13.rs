use crate::utils;

struct Origami {
    grid: Vec<Vec<u32>>,
    folds: Vec<(char, usize)>,
}

impl Origami {
    pub fn new() -> Origami {
        let data = utils::read_lines_to_string("inputs/day13.txt");
        let mut grid = vec![vec![0; 1310]; 895];
        let mut folds = Vec::<(char, usize)>::new();

        let mut folding = false;
        for line in data.lines() {
            if line.is_empty() {
                folding = true;
                continue;
            }

            if !folding {
                let mut split = line.split(",");
                let x = split.next().unwrap().parse::<usize>().unwrap();
                let y = split.next().unwrap().parse::<usize>().unwrap();

                grid[y][x] = 1;
            } else {
                let mut split = line.split("=");
                let axis = split.next().unwrap().chars().last().unwrap();
                let value = split.next().unwrap().parse::<usize>().unwrap();
                folds.push((axis, value));
            }
        }

        Origami { grid, folds }
    }

    pub fn fold(&mut self, axis: char, value: usize) {
        match axis {
            'y' => {
                for y in value..self.grid.len() {
                    for x in 0..self.grid[y].len() {
                        let number = self.grid[y][x];
                        match number {
                            0 => (),
                            1 => {
                                let new_pos = value - (y - value);
                                self.grid[new_pos][x] = 1;
                            }
                            _ => (),
                        }
                    }
                }
                self.grid.resize(value, Vec::default());
            }
            'x' => {
                for x in value..self.grid[0].len() {
                    for y in 0..self.grid.len() {
                        let number = self.grid[y][x];
                        match number {
                            0 => (),
                            1 => {
                                let new_pos = value - (x - value);
                                self.grid[y][new_pos] = 1;
                            }
                            _ => (),
                        }
                    }
                }

                for y in 0..self.grid.len() {
                    self.grid[y].resize(value, 0);
                }
            }
            _ => (),
        }
    }

    pub fn get_dots(&self) -> u32 {
        self.grid.iter().map(|r| r.iter().sum::<u32>()).sum::<u32>()
    }
}

pub fn solve_first_puzzle() {
    let mut origami = Origami::new();

    origami.fold(origami.folds[0].0, origami.folds[0].1);

    println!("D13P1: {}", origami.get_dots());
}

pub fn solve_second_puzzle() {
    let mut origami = Origami::new();

    for (axis, value) in origami.folds.clone() {
        origami.fold(axis, value);
    }

    origami.grid.iter().for_each(|row| {
        println!("{:?}", row);
    });

    println!("D13P2: CEJKLUGJ");
}
