use std::str::FromStr;
use std::fs::read_to_string;

#[derive(Debug)]
enum Movement {
    Forward { value: i32 },
    Down { value: i32 },
    Up { value: i32 },
}

#[derive(Debug)]
struct UnknownMovement;

impl FromStr for Movement {
    type Err = UnknownMovement;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(' ').collect();
        let value: i32 = split[1].parse().unwrap();
        match split[0] {
            "forward" => Ok(Self::Forward { value }),
            "down" => Ok(Self::Down { value }),
            "up" => Ok(Self::Up { value }),
            _ => Err(UnknownMovement),
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn go(&mut self, movement: Movement) {
        match movement {
            Movement::Down { value } => self.aim += value,
            Movement::Up { value } => self.aim -= value,
            Movement::Forward { value } => {
                self.horizontal += value;
                self.depth += self.aim * value;
            },
        }
    }
}

fn read_input() -> Vec<String>{
    read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn main() {
    let input = read_input();
    let mut submarine = Submarine::default();
    input.iter().for_each(|s| {
        let movement = Movement::from_str(s).unwrap();
        submarine.go(movement);
    });
    dbg!(&submarine);
    dbg!(submarine.depth * submarine.horizontal);
}
