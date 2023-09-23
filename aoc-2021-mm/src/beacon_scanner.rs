use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};

pub struct BeaconScanner;

impl AocTask for BeaconScanner {
    fn directory(&self) -> PathBuf {
        "tasks/19_beacon_scanner".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        input.solved()
    }
}
