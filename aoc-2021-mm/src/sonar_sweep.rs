use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;

pub struct SonarSweep;

impl AocTask for SonarSweep {
    fn directory(&self) -> PathBuf {
        "tasks/01_sonar_sweep".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let window_size = match phase {
            1 => 1,
            2 => 3,
            _ => todo!(),
        };

        input
            .map(|str| str.parse())
            .collect::<Result<Vec<_>, _>>()?
            .windows(window_size)
            .map(|win| win.iter().sum::<i32>())
            .tuple_windows()
            .filter(|(old, new)| new > old)
            .count()
            .solved()
    }
}
