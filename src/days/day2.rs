use crate::utils;

pub fn solve() -> i64 {
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;

    let data = utils::read_lines_to_string("inputs/day2.txt");
    let hashmap = data
        .lines()
        .map(|line| utils::split_string(line))
        .collect::<Vec<(&str, i64)>>();
    for (direction, steps) in hashmap {
        match direction {
            "forward" => horizontal += steps,
            "down" => depth += steps,
            "up" => depth -= steps,
            _ => {
                panic!("This path does not exist")
            }
        }
    }

    depth * horizontal
}

pub fn solve_aim() -> i64 {
    let mut depth: i64 = 0;
    let mut horizontal: i64 = 0;
    let mut aim: i64 = 0;

    let data = utils::read_lines_to_string("inputs/day2.txt");
    let hashmap = data
        .lines()
        .map(|line| utils::split_string(line))
        .collect::<Vec<(&str, i64)>>();
    for (direction, steps) in hashmap {
        match direction {
            "forward" => {
                horizontal += steps;
                depth += steps * aim
            }
            "down" => aim += steps,
            "up" => aim -= steps,
            _ => {
                panic!("This path does not exist")
            }
        }
    }

    depth * horizontal
}

pub fn solve_first_puzzle() {
    println!("D2P1: {}", solve())
}

pub fn solve_second_puzzle() {
    println!("D2P2: {}", solve_aim())
}
