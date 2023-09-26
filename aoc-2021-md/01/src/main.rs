use std::error::Error;
use std::fs::read_to_string;

fn main() {
    let input = read_input();
    dbg!(&input);
    let solution = solve(input);
    dbg!(solution);
}

fn read_input() -> Vec<i32>{
    read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn solve(input: Vec<i32>) -> usize {
    input
        .windows(4)
        .filter(|v| v[0] < v[3])
        .count()
}
