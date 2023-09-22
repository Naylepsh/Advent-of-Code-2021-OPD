use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct ReactorReboot;

impl AocTask for ReactorReboot {
    fn directory(&self) -> PathBuf {
        "tasks/22_reactor_reboot".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
