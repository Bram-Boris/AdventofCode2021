use crate::utils;

pub fn solve_first_puzzle() {
    let mut fuel: isize = isize::max_value();
    let data = utils::read_lines_to_string("inputs/day7.txt");
    let raw_crabs = data
            .split(",")
            .map(|c| c.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

    for pos in 0..*raw_crabs.iter().max().unwrap()
    {
        let fuel_for_pos = raw_crabs.iter().map(|value| isize::abs(pos - value)).sum::<isize>();
        if fuel_for_pos < fuel {
            fuel = fuel_for_pos;
        }
    }

    println!("D7P1: {:?}", fuel);
}

pub fn solve_second_puzzle() {
    let mut fuel: isize = isize::max_value();
    let data = utils::read_lines_to_string("inputs/day7.txt");
    let raw_crabs = data
            .split(",")
            .map(|c| c.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

for pos in 0..*raw_crabs.iter().max().unwrap()
    {
        let fuel_for_pos = raw_crabs.iter().map(|value| {
            let pos = isize::abs(pos - value);
            pos * (pos + 1) / 2
        }).sum::<isize>();
        if fuel_for_pos < fuel {
            fuel = fuel_for_pos;
        }
    }

    println!("D7P1: {:?}", fuel);
}
