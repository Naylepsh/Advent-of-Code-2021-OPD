use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct SeaCucumber;

impl AocTask for SeaCucumber {
    fn directory(&self) -> PathBuf {
        "tasks/25_sea_cucumber".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
