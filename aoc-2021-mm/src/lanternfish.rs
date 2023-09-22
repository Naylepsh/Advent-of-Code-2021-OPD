use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct Lanternfish;

impl AocTask for Lanternfish {
    fn directory(&self) -> PathBuf {
        "tasks/06_lanternfish".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
