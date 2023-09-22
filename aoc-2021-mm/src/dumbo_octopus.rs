use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct DumboOctopus;

impl AocTask for DumboOctopus {
    fn directory(&self) -> PathBuf {
        "tasks/11_dumbo_octopus".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
