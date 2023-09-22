use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct SmokeBasin;

impl AocTask for SmokeBasin {
    fn directory(&self) -> PathBuf {
        "tasks/09_smoke_basin".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
