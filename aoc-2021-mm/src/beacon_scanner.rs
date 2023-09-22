use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct BeaconScanner;

impl AocTask for BeaconScanner {
    fn directory(&self) -> PathBuf {
        "tasks/19_beacon_scanner".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
