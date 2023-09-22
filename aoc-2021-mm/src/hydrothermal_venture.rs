use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct HydrothermalVenture;

impl AocTask for HydrothermalVenture {
    fn directory(&self) -> PathBuf {
        "tasks/05_hydrothermal_venture".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
