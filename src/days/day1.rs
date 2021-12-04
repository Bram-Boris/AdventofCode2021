use crate::utils;

pub fn solve(window: usize) -> i32 {
    let data = utils::read_lines_to_ints("inputs/day1.txt");
    let mut increments = 0;
    let mut previous_sum = 0;
    for slice in data.windows(window) {
        let sum: i32 = slice.iter().sum();
        if previous_sum != 0 && previous_sum < sum {
            increments = increments + 1;
        }

        previous_sum = sum;
    }

    increments
}

pub fn solve_first_puzzle() {
    println!("D1P1: {}", solve(1))
}

pub fn solve_second_puzzle() {
    println!("D1P2: {}", solve(3))
}
