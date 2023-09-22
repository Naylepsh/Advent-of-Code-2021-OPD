use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct TrickShot;

impl AocTask for TrickShot {
    fn directory(&self) -> PathBuf {
        "tasks/17_trick_shot".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
