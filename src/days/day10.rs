use crate::utils;
use std::{collections::HashMap, ops::Index};

pub fn solve_first_puzzle() {
    let data = utils::read_lines_to_string("inputs/day10.txt");
    let mut closing = HashMap::new();
    closing.insert(')', '(');
    closing.insert(']', '[');
    closing.insert('}', '{');
    closing.insert('>', '<');

    let mut opening = HashMap::new();
    opening.insert('(', ')');
    opening.insert('[', ']');
    opening.insert('{', '}');
    opening.insert('<', '>');

    let mut total_points = 0;
    for line in data.lines() {
        let mut last_opening: Vec<char> = Vec::new();
        let mut corrupted = false;
        let mut lines = line.chars();
        while let Some(c) = lines.next() {
            if corrupted {
                continue;
            }
            match c {
                '(' | '[' | '{' | '<' => last_opening.push(c),
                ')' | ']' | '}' | '>' => {
                    if *last_opening.last().unwrap() == *closing.get(&c).unwrap() {
                        last_opening.pop();
                    } else {
                        corrupted = true;
                        match c {
                            ')' => total_points += 3,
                            ']' => total_points += 57,
                            '}' => total_points += 1197,
                            '>' => total_points += 25137,
                            _ => (),
                        }
                    }
                }
                _ => println!("Unknown char? {}", c),
            }
        }
    }
    println!("D10P1: {}", total_points);
}

pub fn solve_second_puzzle() {
    let data = utils::read_lines_to_string("inputs/day10.txt");
    let mut closing = HashMap::new();
    closing.insert(')', '(');
    closing.insert(']', '[');
    closing.insert('}', '{');
    closing.insert('>', '<');

    let mut opening = HashMap::new();
    opening.insert('(', ')');
    opening.insert('[', ']');
    opening.insert('{', '}');
    opening.insert('<', '>');

    let mut scores = Vec::<u64>::new();
    for line in data.lines() {
        let mut last_opening: Vec<char> = Vec::new();
        let mut corrupted = false;
        let mut lines = line.chars();
        while let Some(c) = lines.next() {
            if corrupted {
                continue;
            }
            match c {
                '(' | '[' | '{' | '<' => last_opening.push(c),
                ')' | ']' | '}' | '>' => {
                    if *last_opening.last().unwrap() == *closing.get(&c).unwrap() {
                        last_opening.pop();
                    } else {
                        corrupted = true;
                    }
                }
                _ => println!("Unknown char? {}", c),
            }
        }

        // Line is incomplete
        if last_opening.len() > 0 && !corrupted {
            let mut score: u64 = 0;
            for c in last_opening.iter().rev() {
                let open = opening.get(&c).unwrap();
                score *= 5;
                match open {
                    ')' => score += 1,
                    ']' => score += 2,
                    '}' => score += 3,
                    '>' => score += 4,
                    _ => (),
                }
            }
            scores.push(score);
        }
    }
    scores.sort();
    let middle: f32 = (scores.len() - 1) as f32 / 2.0_f32;
    let index_middle = middle.ceil() as usize;
    println!("D10P2: {}", scores.index(index_middle));
}
