use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Bingo {
    numbers: Vec<Vec<usize>>
}

impl Bingo {
    fn find_number(&self, search_term: usize) -> Vec<(usize, usize)> {
        let mut positions: Vec<(usize, usize)> = vec![];
        self.numbers
            .iter()
            .enumerate()
            .for_each(|(x, row)| row
                .iter()
                .enumerate()
                .for_each(|(y, &number)| {
                    if number == search_term {
                        positions.push((x, y));
                    }
                })
            );
        positions
    }
}

#[derive(Debug, Clone)]
struct Markers {
    numbers: Vec<Vec<usize>>
}

impl Markers {
    fn mark(&mut self, (x, y): (usize, usize)) {
        self.numbers[x][y] = 1;
    }

    fn is_solved(&self) -> bool {
        // if any row is all ones
        if self.numbers.iter().any(|row| row.iter().all(|&number| number == 1)) {
            return true
        }

        // if any column is all ones
        for col in 0..5 {
            if self.numbers.iter().all(|row| row[col] == 1) {
                return true
            }
        }

        false
    }


}

#[derive(Debug, Clone)]
struct Board {
    bingo: Bingo,
    markers: Markers,
}

impl Board {
    fn mark(&mut self, number: usize) {
        self.bingo
            .find_number(number)
            .iter()
            .for_each(|&position| self.markers.mark(position))
    }

    fn is_solved(&self) -> bool {
        self.markers.is_solved()
    }

    fn get_sum_of_unmarked(&self) -> usize {
        let mut result = 0;
        for col in 0..5 {
            for row in 0..5 {
                if self.markers.numbers[row][col] == 0 {
                    result += self.bingo.numbers[row][col];
                }
            }
        }
        result
    }
}

impl From<String> for Board {
    fn from(value: String) -> Self {
        let numbers: Vec<Vec<usize>> = value.
            split('\n')
            .map(|row|
                row
                    .split_whitespace()
                    .map(|number| number.parse().unwrap())
                    .collect()
            )
            .collect();
        Self {
            bingo: Bingo { numbers },
            markers: Markers { numbers:  vec![vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]] },
        }
    }
}

fn main() {
    let (numbers, mut boards) = read_input();
    solve_first(numbers, &mut boards);
}

fn solve_first(numbers: Vec<usize>, boards: &mut [Board]) {
    fn get_winning_board_and_number(numbers: Vec<usize>, boards: &mut [Board]) -> Option<(Board, usize)> {
        for number in numbers {
            for board in &mut boards.iter_mut() {
                board.mark(number);
                if board.is_solved() {
                    return Some((board.clone(), number));
                }
            }
        }
        None
    }

    if let Some((winning_board, winning_number)) = get_winning_board_and_number(numbers, boards) {
        println!("{}", winning_board.get_sum_of_unmarked() * winning_number);
    } else {
        println!("No winners this time!");
    }
}

fn read_input() -> (Vec<usize>, Vec<Board>) {
    let x = read_to_string("input").unwrap();
    let numbers: Vec<usize> = x
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();
    let mut grids: Vec<String> = x
        .split("\n\n")
        .map(String::from)
        .collect();
    grids.remove(0);
    (
        numbers,
        grids.into_iter().map(Board::from).collect::<Vec<_>>(),
    )
}
