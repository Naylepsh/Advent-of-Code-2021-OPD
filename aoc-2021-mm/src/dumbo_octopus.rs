use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct DumboOctopus;

impl AocTask for DumboOctopus {
    fn directory(&self) -> PathBuf {
        "tasks/11_dumbo_octopus".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
