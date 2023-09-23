use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct SevenSegmentSearch;

impl AocTask for SevenSegmentSearch {
    fn directory(&self) -> PathBuf {
        "tasks/08_seven_segment_search".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
