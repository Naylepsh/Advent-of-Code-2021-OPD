use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct SyntaxScoring;

impl AocTask for SyntaxScoring {
    fn directory(&self) -> PathBuf {
        "tasks/10_syntax_scoring".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
