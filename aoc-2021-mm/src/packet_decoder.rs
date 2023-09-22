use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{AocInput, AocSolution, AocTask, traits::*};

pub struct PacketDecoder;

impl AocTask for PacketDecoder {
    fn directory(&self) -> PathBuf {
        "tasks/16_packet_decoder".into()
    }

    fn solution(&self, input: AocInput, phase: usize) -> Result<AocSolution, BoxedError> {
        input.flatten().solved()
    }
}
