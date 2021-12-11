use crate::utils;

struct Octopi {
    grid: Vec<Vec<u32>>,
    flashed: Vec<(usize, usize)>,
    flashes: u32,
}

impl Octopi {
    pub fn new() -> Octopi {
        let data = utils::read_lines_to_string("inputs/day11.txt");
        let grid = data
            .lines()
            .map(|l| {
                l.split("")
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect::<Vec<Vec<u32>>>();

        let flashed = Vec::<(usize, usize)>::new();

        Octopi {
            grid,
            flashed,
            flashes: 0,
        }
    }
    pub fn step(&mut self) -> u32 {
        self.flashed.clear();
        self.flashes = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                self.increase(x, y);
            }
        }

        self.flashes
    }

    pub fn flash(&mut self, x: usize, y: usize) {
        self.flashes += 1;
        //Top
        if y as isize - 1 >= 0 {
            self.increase(x, y - 1);

            // Top right
            if x + 1 < self.grid.len() {
                self.increase(x + 1, y - 1);
            }
            // Top left
            if x as isize - 1 >= 0 {
                self.increase(x - 1, y - 1);
            }
        }

        //Bottom
        if y + 1 < self.grid.len() {
            self.increase(x, y + 1);

            // Bottom right
            if x + 1 < self.grid.len() {
                self.increase(x + 1, y + 1);
            }
            // Bottom left
            if x as isize - 1 >= 0 {
                self.increase(x - 1, y + 1);
            }
        }

        //Left
        if x as isize - 1 >= 0 {
            self.increase(x - 1, y);
        }
        //Right
        if x + 1 < self.grid.len() {
            self.increase(x + 1, y);
        }
    }

    pub fn increase(&mut self, x: usize, y: usize) {
        if !self.flashed.contains(&(x, y)) {
            self.grid[y][x] += 1;
            if self.grid[y][x] > 9 {
                self.grid[y][x] = 0;
                self.flashed.push((x, y));
                self.flash(x as usize, y as usize);
            }
        }
    }
}

pub fn solve_first_puzzle() {
    let mut flashes = 0;
    let mut octopi = Octopi::new();
    for _ in 0..100 {
        flashes += octopi.step();
    }

    println!("D11P1: {}", flashes);
}

pub fn solve_second_puzzle() {
    let mut octopi = Octopi::new();

    let mut step = 0;
    while octopi.grid.iter().map(|r| r.iter().sum::<u32>()).sum::<u32>() != 0 {
        step += 1;
        octopi.step();
    }

    println!("D11P2: {}", step);
}
