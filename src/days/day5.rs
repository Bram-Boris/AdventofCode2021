use crate::utils;

struct Pipeline {
    data: String,
    lines: Vec<Vec<u32>>
}

impl Pipeline {
    pub fn new() -> Pipeline
    {
        let data = utils::read_lines_to_string("inputs/day5.txt");
        let lines = vec![vec![0; 1000]; 1000];

        Pipeline { lines,data }
    }

    pub fn mark_pipelines(&mut self, diag: bool) {
         for line in  self.data.lines(){
            let split = line.split(" -> ").collect::<Vec<&str>>();
            let from = split[0].split(",").collect::<Vec<&str>>();
            let to = split[1].split(",").collect::<Vec<&str>>();

            let from_x = from[0].parse::<usize>().expect("It is a digit.");
            let from_y = from[1].parse::<usize>().expect("It is a digit.");
            let to_x = to[0].parse::<usize>().expect("It is a digit.");
            let to_y = to[1].parse::<usize>().expect("It is a digit.");

            // Horizontal
            if from_y == to_y {
                for x in utils::get_iterator_abs(from_x, to_x) {
                    self.lines[from_y][x] += 1;
                }
            } else if from_x == to_x {
                for y in utils::get_iterator_abs(from_y, to_y) {
                    self.lines[y][from_x] += 1;
                }
            } else {
                if diag {
                    // Direction is now from left to right, top to bottom.
                    let range_x = utils::get_iterator_diag(from_x, to_x);
                    let range_y = utils::get_iterator_diag(from_y, to_y);
                    for (x,y) in range_x.zip(range_y) {
                        self.lines[x][y] += 1;
                    }
                }

            }
         }
    }

    pub fn get_overlapping(&self) -> u32 {
        let mut overlap = 0;
        self.lines.iter().for_each(|row| row.iter().for_each(|digit|
                                                             {
                                                                 if *digit >= 2 {
                                                                     overlap += 1;
                                                                 }
                                                             }));

        overlap
    }
}


pub fn solve_first_puzzle() {
    let mut pipeline = Pipeline::new();
    pipeline.mark_pipelines(false);
    println!("D5P1: {}", pipeline.get_overlapping());
}

pub fn solve_second_puzzle() {
    let mut pipeline = Pipeline::new();
    pipeline.mark_pipelines(true);

    println!("D5P2: {}", pipeline.get_overlapping());
}
