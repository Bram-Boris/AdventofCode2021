use std::ops::Index;

use crate::utils;

pub fn solve() -> u32 {
    let data = utils::read_lines_to_string("inputs/day3.txt");
    let mut chars = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    chars = utils::transpose(chars);
    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();

    for ele in chars {
        let sum = ele.iter().sum::<u32>();
        let len = ele.len() as u32;
        let zeroes: u32 = len - sum;
        let ones: u32 = len - zeroes;

        if zeroes > ones {
            gamma_rate.push_str("0");
            epsilon_rate.push_str("1");
        } else {
            epsilon_rate.push_str("0");
            gamma_rate.push_str("1")
        }
    }

    u32::from_str_radix(gamma_rate.as_str(), 2).unwrap()
        * u32::from_str_radix(epsilon_rate.as_str(), 2).unwrap()
}

pub fn solve_life_support() -> u32 {
    let data = utils::read_lines_to_string("inputs/day3.txt");
    let mut oxygen = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut co2 = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut oxygen_result: String = String::new();
    let mut co2_result: String = String::new();
    let mut oxygen_found = false;
    let mut co2_found = false;

    let mut index = 0;
    while !oxygen_found {
        let transposed = utils::transpose(oxygen.clone());
        let sum = transposed.index(index).iter().sum::<u32>();
        let len = transposed.index(index).len() as u32;
        let zeroes: u32 = len - sum;
        let ones: u32 = len - zeroes;

        if ones > zeroes {
            oxygen.retain(|x| *x.index(index) == 1 as u32);
        } else if zeroes > ones {
            oxygen.retain(|x| *x.index(index) == 0 as u32);
        } else {
            oxygen.retain(|x| *x.index(index) == 1 as u32);
        }

        if oxygen.len() == 1 {
            oxygen_result = oxygen
                .first()
                .unwrap()
                .iter()
                .map(|x| x.to_string())
                .collect::<String>();
            oxygen_found = true;
        }

        index += 1;
    }
    index = 0;
    while !co2_found {
        let transposed = utils::transpose(co2.clone());
        let sum = transposed.index(index).iter().sum::<u32>();
        let len = transposed.index(index).len() as u32;
        let zeroes: u32 = len - sum;
        let ones: u32 = len - zeroes;

        if ones > zeroes {
            co2.retain(|x| *x.index(index) == 0 as u32);
        } else if zeroes > ones {
            co2.retain(|x| *x.index(index) == 1 as u32);
        } else {
            co2.retain(|x| *x.index(index) == 0 as u32);
        }

        if co2.len() == 1 {
            co2_result = co2
                .first()
                .unwrap()
                .iter()
                .map(|x| x.to_string())
                .collect::<String>();
            co2_found = true;
        }

        index += 1;
    }

    u32::from_str_radix(oxygen_result.as_str(), 2).unwrap()
        * u32::from_str_radix(co2_result.as_str(), 2).unwrap()
}

pub fn solve_first_puzzle() {
    println!("D3P1: {}", solve())
}

pub fn solve_second_puzzle() {
    println!("D3P2: {}", solve_life_support())
}
