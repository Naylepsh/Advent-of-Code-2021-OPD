use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct BinaryDiagnostic;

impl AocTask for BinaryDiagnostic {
    fn directory(&self) -> PathBuf {
        "tasks/03_binary_diagnostic".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
