use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn compute_complex_rating(
    lines: Vec<String>,
    i: usize,
    determine_bit: fn(i32, i32) -> char,
) -> u32 {
    if lines.len() == 1 {
        isize::from_str_radix(lines[0].as_str(), 2).unwrap() as u32
    } else {
        let (zeros, ones) =
            lines
                .iter()
                .fold((0, 0), |(zeros, ones), line| match line.chars().nth(i) {
                    Some('0') => (zeros + 1, ones),
                    Some('1') => (zeros, ones + 1),
                    _ => (zeros, ones),
                });
        let bit_value = determine_bit(zeros, ones);

        let leftover = lines
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == bit_value)
            .collect::<Vec<_>>();

        compute_complex_rating(leftover, i + 1, determine_bit)
    }
}

fn determine_oxygen_bit(zeros: i32, ones: i32) -> char {
    if ones >= zeros {
        '1'
    } else {
        '0'
    }
}

fn determine_co2_bit(zeros: i32, ones: i32) -> char {
    if ones >= zeros {
        '0'
    } else {
        '1'
    }
}

fn solve_first(lines: Vec<String>) -> i32 {
    let length = lines.first().map(|line| line.len()).unwrap_or(0);
    let mut counts = (0..length).map(|_| (0, 0)).collect::<Vec<_>>();
    lines.iter().for_each(|line| {
        line.chars().enumerate().for_each(|(i, bit)| {
            let (zeros, ones) = counts[i];
            let result = match bit {
                '0' => (zeros + 1, ones),
                '1' => (zeros, ones + 1),
                _ => (zeros, ones),
            };
            counts[i] = result;
        })
    });

    let mut gamma = 0;
    let mut epsilon = 0;
    counts.iter().for_each(|(ones, zeros)| {
        if ones > zeros {
            gamma = (gamma << 1) + 1;
            epsilon <<= 1;
        } else {
            gamma <<= 1;
            epsilon = (epsilon << 1) + 1;
        };
    });

    gamma * epsilon
}

fn solve_second(lines: Vec<String>) -> u32 {
    let oxy = compute_complex_rating(lines.clone(), 0, determine_oxygen_bit);
    let co2 = compute_complex_rating(lines, 0, determine_co2_bit);

    oxy * co2
}

fn main() {
    let lines = read_lines("./input.txt");
    // let lines = read_lines("./example.txt");

    println!("first: {}", solve_first(lines.clone()));
    println!("second: {}", solve_second(lines));
}
