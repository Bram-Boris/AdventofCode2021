use crate::utils;

pub fn is_lower(number: isize, number2: Option<isize>) -> bool {
    match number2 {
        Some(x) => number < x,
        None => true,
    }
}

pub fn solve_first_puzzle() {
    let data = utils::read_lines_to_string("inputs/day9.txt");
    let mut heightmap = vec![vec![0 as isize; 100]; 100];
    let mut lowest_values = Vec::<isize>::new();
    for (index, d) in data.lines().enumerate() {
        heightmap[index] = d
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
    }

    for (index_y, y) in heightmap.iter().enumerate() {
        for (index_x, _x) in y.iter().enumerate() {
            let current_value = heightmap[index_y][index_x];
            let mut top: Option<isize> = None;
            let mut bottom: Option<isize> = None;
            let mut left: Option<isize> = None;
            let mut right: Option<isize> = None;
            let index_y_signed = index_y as isize;
            let index_x_signed = index_x as isize;

            //println!("Current pos x: {} y: {} value: {}", index_x, index_y, x);

            //Top
            if index_y_signed - 1 >= 0 {
                top = Some(heightmap[index_y - 1][index_x])
            }
            //Bottom
            if index_y_signed + 1 < heightmap.len() as isize {
                bottom = Some(heightmap[index_y + 1][index_x])
            }
            //Left
            if index_x_signed - 1 >= 0 {
                left = Some(heightmap[index_y][index_x - 1])
            }
            //Right
            if index_x_signed + 1 < y.len() as isize {
                right = Some(heightmap[index_y][index_x + 1])
            }

            let current_value_is_lowest_point = is_lower(current_value, top)
                && is_lower(current_value, bottom)
                && is_lower(current_value, left)
                && is_lower(current_value, right);
            if current_value_is_lowest_point {
                lowest_values.push(current_value + 1);
            }
        }
    }

    println!("D9P1: {:?}", lowest_values.iter().sum::<isize>());
}

pub fn solve_second_puzzle() {

    // println!("D6P2: {:?}", school.fishies());
}
