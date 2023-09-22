use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn compute_oxygen(lines: Vec<String>, i: usize) -> u32 {
    if lines.len() == 1 {
        isize::from_str_radix(lines[0].as_str(), 2).unwrap() as u32
    } else {
        let mut zeros = 0;
        let mut ones = 0;
        lines.iter().for_each(|line| {
            match line.chars().nth(i).unwrap() {
                '0' => zeros += 1,
                '1' => ones += 1,
                _ => (),
            };
        });
        let value = if ones >= zeros { '1' } else { '0' };
        let leftover = lines
            .into_iter()
            .filter(|line| line.chars().nth(i).unwrap() == value)
            .collect::<Vec<_>>();
        compute_oxygen(leftover, i + 1)
    }
}

fn main() {
    let lines = read_lines("./input.txt");
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
            epsilon = (epsilon << 1) + 0;
        } else {
            gamma = (gamma << 1) + 0;
            epsilon = (epsilon << 1) + 1;
        };
    });
    let mut oxygen_candidates = counts.clone();
    let mut i = 0;
    let result = gamma * epsilon;

    println!("g:{} e:{} r:{}", gamma, epsilon, result);
}
