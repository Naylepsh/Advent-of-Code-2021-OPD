use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct TheTreacherOfWhales;

impl AocTask for TheTreacherOfWhales {
    fn directory(&self) -> PathBuf {
        "tasks/07_the_treacher_of_whales".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
