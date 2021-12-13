use std::collections::HashMap;

use crate::utils;

struct SegmentDisplay {
    input: Vec<String>,
    output: Vec<String>,
    mapping: HashMap<u32, String>,
    top: String,
    _middle: String,
    _bottom: String,
    top_right: String,
    bottom_right: String,
    _top_left: String,
    bottom_left: String,
}

impl SegmentDisplay {
    pub fn new(input: String) -> SegmentDisplay {
        let split = input
            .split(" | ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        SegmentDisplay {
            input: split[0]
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            output: split[1]
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            mapping: HashMap::new(),
            top: String::new(),
            _middle: String::new(),
            _bottom: String::new(),
            top_right: String::new(),
            bottom_right: String::new(),
            _top_left: String::new(),
            bottom_left: String::new(),
        }
    }

    pub fn parse_unique_digits(&mut self) {
        self.input.iter().enumerate().for_each(|(_index, number)| {
            match number.len() {
                2 => self.mapping.insert(1, number.to_string()),
                4 => self.mapping.insert(4, number.to_string()),
                3 => self.mapping.insert(7, number.to_string()),
                7 => self.mapping.insert(8, number.to_string()),
                _ => None,
            };
        })
    }

    pub fn generate_remaining_digits(&mut self) {
        // Generate top
        let one = self.mapping.get(&1).unwrap().clone();
        let mut top = self.mapping.get(&7).unwrap().clone();
        one.chars().for_each(|c| top = top.replace(c, ""));
        self.top = top;

        // Determine 6 and top_right / bottom_right
        let candidates = self.input.iter().filter(|s| s.len() == 6);
        let mut top_right = ' ';
        let six = candidates
            .filter(|c| {
                one.chars().any(|o| {
                    if !c.contains(o) {
                        top_right = o;
                        return true;
                    }
                    false
                })
            })
            .next()
            .unwrap();

        self.mapping.insert(6, six.to_string());
        self.top_right = top_right.to_string();
        self.bottom_right = one.clone().replace(top_right, "");

        // Determine 2,3,5
        let candidates = self.input.iter().filter(|s| s.len() == 5);
        for c in candidates {
            if !c.contains(&self.top_right) {
                self.mapping.insert(5, c.to_string());
            } else if c.contains(&self.top_right) && c.contains(&self.bottom_right) {
                self.mapping.insert(3, c.to_string());
            } else {
                self.mapping.insert(2, c.to_string());
            }
        }

        // Determine bottom left to calculate the 0 and the 9.
        let three = self.mapping.get(&3).unwrap().clone();
        let mut bottom_left = self.mapping.get(&2).unwrap().clone();
        three
            .chars()
            .for_each(|c| bottom_left = bottom_left.replace(c, ""));
        self.bottom_left = bottom_left;

        let candidates = self.input.iter().filter(|s| s.len() == 6);
        for c in candidates {
            if !c.contains(&self.bottom_left) {
                self.mapping.insert(9, c.to_string());
            } else if c.contains(&self.bottom_left) && c.contains(&self.top_right) {
                self.mapping.insert(0, c.to_string());
            }
        }
    }

    pub fn get_p1_digits(&self) -> usize {
        let mut sum = 0;
        self.get_output().to_string().chars().for_each(|c| match c {
            '1' | '4' | '7' | '8' => sum += 1,
            _ => (),
        });
        sum
    }

    pub fn get_output(&self) -> u32 {
        let mut number = String::new();
        for o in &self.output {
            for (key, value) in &self.mapping {
                let mut vec_o = o.chars().collect::<Vec<char>>();
                let mut vec_value = value.chars().collect::<Vec<char>>();
                vec_o.sort();
                vec_value.sort();
                if vec_o.iter().collect::<String>() == vec_value.iter().collect::<String>() {
                    number.push_str(&key.to_string());
                }
            }
        }
        number.parse::<u32>().unwrap()
    }
}

pub fn solve_first_puzzle() {
    let data = utils::read_lines_to_string("inputs/day8.txt");
    let mut values = data
        .lines()
        .map(|line| SegmentDisplay::new(line.to_string()))
        .collect::<Vec<SegmentDisplay>>();

    values
        .iter_mut()
        .for_each(|display| display.parse_unique_digits());
    values
        .iter_mut()
        .for_each(|display| display.generate_remaining_digits());
    let count = values
        .iter()
        .map(|display| display.get_p1_digits())
        .sum::<usize>();

    println!("D8P1: {:?}", count);
}

pub fn solve_second_puzzle() {
    let data = utils::read_lines_to_string("inputs/day8.txt");
    let mut values = data
        .lines()
        .map(|line| SegmentDisplay::new(line.to_string()))
        .collect::<Vec<SegmentDisplay>>();

    values
        .iter_mut()
        .for_each(|display| display.parse_unique_digits());
    values
        .iter_mut()
        .for_each(|display| display.generate_remaining_digits());
    let count = values
        .iter()
        .map(|display| display.get_output())
        .sum::<u32>();

    println!("D8P2: {:?}", count);
}
