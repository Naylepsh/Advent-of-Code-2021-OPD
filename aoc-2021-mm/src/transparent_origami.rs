use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct TransparentOrigami;

impl AocTask for TransparentOrigami {
    fn directory(&self) -> PathBuf {
        "tasks/13_transparent_origami".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
