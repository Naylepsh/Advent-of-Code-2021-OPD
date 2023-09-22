use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct SevenSegmentSearch;

impl AocTask for SevenSegmentSearch {
    fn directory(&self) -> PathBuf {
        "tasks/08_seven_segment_search".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
