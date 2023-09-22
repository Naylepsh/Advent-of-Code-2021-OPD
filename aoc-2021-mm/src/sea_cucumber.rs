use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct SeaCucumber;

impl AocTask for SeaCucumber {
    fn directory(&self) -> PathBuf {
        "tasks/25_sea_cucumber".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
