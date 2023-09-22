use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct SyntaxScoring;

impl AocTask for SyntaxScoring {
    fn directory(&self) -> PathBuf {
        "tasks/10_syntax_scoring".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
