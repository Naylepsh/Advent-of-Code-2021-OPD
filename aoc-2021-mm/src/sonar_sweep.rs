use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocInput, AocSolution, AocTask};
use itertools::Itertools;

pub struct SonarSweep;

impl AocTask for SonarSweep {
    fn directory(&self) -> PathBuf {
        "tasks/01_sonar_sweep".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        let window_size = match phase {
            1 => 1,
            2 => 3,
            _ => todo!(),
        };

        input
            .flatten()
            .map(|str| str.parse())
            .try_collect::<_, Vec<i32>, _>()?
            .windows(window_size)
            .fold((0i32, None), |(increases, last), input| {
                match (last, input.iter().sum::<i32>()) {
                    (Some(a), b) if b > a => (increases + 1, Some(b)),
                    (_, b) => (increases, Some(b)),
                }
            })
            .0
            .solved()
    }
}
