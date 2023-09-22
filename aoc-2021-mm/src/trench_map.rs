use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct TrenchMap;

impl AocTask for TrenchMap {
    fn directory(&self) -> PathBuf {
        "tasks/20_trench_map".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
