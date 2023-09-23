use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct ArithmeticLogicUnit;

impl AocTask for ArithmeticLogicUnit {
    fn directory(&self) -> PathBuf {
        "tasks/24_arithmetic_logic_unit".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
