use std::fs::read_to_string;
use std::ops::Add;

#[derive(Debug)]
struct Diagnosis {
    val: Vec<i32>
}

impl From<&String> for Diagnosis {
    fn from(value: &String) -> Self {
        let val= value.chars().map(|s| s.to_string().parse().unwrap()).collect();
        Self { val }
    }
}

impl Add for Diagnosis {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            val: self.val.iter().zip(rhs.val.iter()).map(|(&v, &w)| v + w).collect()
        }
    }
}

fn bool_vec_to_i32(bool_vec: Vec<bool>) -> i32 {
    let mut result = 0;
    for &bit in bool_vec.iter() {
        result <<= 1;
        if bit {
            result |= 1;
        }
    }
    result
}

fn main() {
    let values: Vec<Diagnosis> = read_input().iter().map(Diagnosis::from).collect();
    let num_of_values: i32 = values.len() as i32;
    let final_diagnosis = values.into_iter().reduce(|a, b| a + b).unwrap();
    let gamma = bool_vec_to_i32(
        final_diagnosis.val
        .iter()
        .map(|v| v > &(&num_of_values / 2))
        .collect()
    );
    let epsilon = bool_vec_to_i32(
        final_diagnosis.val
            .iter()
            .map(|v| v < &(&num_of_values / 2))
            .collect()
    );

    dbg!(gamma * epsilon);
}

fn read_input() -> Vec<String> {
    read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
