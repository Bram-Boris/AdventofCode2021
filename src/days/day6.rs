use crate::utils;

#[derive(Debug)]
struct Fish {
    age: i32,
    current_fishies: u64,
}

impl Fish {
    pub fn new(age: i32) -> Fish {
        Fish {
            age,
            current_fishies: 0,
        }
    }
}

struct School {
    fishies: Vec<Fish>,
}

impl School {
    pub fn new() -> School {
        let data = utils::read_lines_to_string("inputs/day6.txt");
        let raw_fishies = data
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut fishies = Vec::<Fish>::new();
        for age in 0..9 {
            fishies.push(Fish::new(age))
        }
        for age in raw_fishies {
            fishies[age].current_fishies += 1;
        }

        School { fishies }
    }

    pub fn advance_day(&mut self) {
        let birthed_fishies = self.fishies[0].current_fishies;
        for fish_age in 0..8 {
            self.fishies[fish_age].current_fishies = self.fishies[fish_age + 1].current_fishies;

        }

        self.fishies[8].current_fishies = birthed_fishies;
        self.fishies[6].current_fishies += birthed_fishies;
    }

    pub fn fishies(&self) -> u64 {
        self.fishies.iter().map(|f| f.current_fishies).sum::<u64>()
    }
}

pub fn solve_first_puzzle() {
    let mut school = School::new();

    for _ in 0..80 {
        school.advance_day();
    }

    println!("D6P1: {:?}", school.fishies());
}

pub fn solve_second_puzzle() {
    let mut school = School::new();

    for _ in 0..256 {
        school.advance_day();
    }

    println!("D6P2: {:?}", school.fishies());
}
