use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct ReactorReboot;

impl AocTask for ReactorReboot {
    fn directory(&self) -> PathBuf {
        "tasks/22_reactor_reboot".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
