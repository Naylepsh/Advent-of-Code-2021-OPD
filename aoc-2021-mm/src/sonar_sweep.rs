use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct SonarSweep;

impl AocTask for SonarSweep {
    fn directory(&self) -> PathBuf {
        "tasks/01_sonar_sweep".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
