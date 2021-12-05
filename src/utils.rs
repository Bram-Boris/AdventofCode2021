use std::{fs, panic};
use either::*;

pub fn read_lines_to_string(filename: &str) -> String {
    fs::read_to_string(filename).expect("Unable to read file")
}

pub fn read_lines_to_ints(filename: &str) -> Vec<i32> {
    let data = read_lines_to_string(filename);
    match data.lines().map(|line| line.parse::<i32>()).collect() {
        Ok(data) => data,
        _ => panic!("Error occurred when parsing ints"),
    }
}

// TODO: Multiple args?
pub fn split_string(line: &str) -> (&str, i64) {
    let split = line.split(" ").collect::<Vec<&str>>();
    (split[0], split[1].parse::<i64>().unwrap())
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn get_iterator_abs(low: usize, high: usize) -> std::ops::Range<usize> {
    if high > low {
        low..high + 1
    } else {
        high..low + 1
    }
}

pub fn get_iterator_diag(low: usize, high: usize) -> either::Either<std::iter::Rev<std::ops::Range<usize>>, std::ops::Range<usize>> {
    if high > low {
        Either::Left((low..high + 1).rev())
    } else {
        Either::Right(high..low + 1)
    }
}
