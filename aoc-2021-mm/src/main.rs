#![feature(lint_reasons)]
#![expect(unused_variables)]

mod amphipod;
mod arithmetic_logic_unit;
mod beacon_scanner;
mod binary_diagnostic;
mod chiton;
mod dirac_dice;
mod dive;
mod dumbo_octopus;
mod extended_polymerization;
mod giant_squid;
mod hydrothermal_venture;
mod lanternfish;
mod packet_decoder;
mod passage_pathing;
mod reactor_reboot;
mod rock_paper_scissors;
mod sea_cucumber;
mod seven_segment_search;
mod smoke_basin;
mod snailfish;
mod sonar_sweep;
mod syntax_scoring;
mod the_treacher_of_whales;
mod transparent_origami;
mod trench_map;
mod trick_shot;

use amphipod::Amphipod;
use arithmetic_logic_unit::ArithmeticLogicUnit;
use beacon_scanner::BeaconScanner;
use binary_diagnostic::BinaryDiagnostic;
use chiton::Chiton;
use dirac_dice::DiracDice;
use dive::Dive;
use dumbo_octopus::DumboOctopus;
use extended_polymerization::ExtendedPolymerization;
use giant_squid::GiantSquid;
use hydrothermal_venture::HydrothermalVenture;
use lanternfish::Lanternfish;
use packet_decoder::PacketDecoder;
use passage_pathing::PassagePathing;
use reactor_reboot::ReactorReboot;
use rock_paper_scissors::RockPaperScissors;
use sea_cucumber::SeaCucumber;
use seven_segment_search::SevenSegmentSearch;
use smoke_basin::SmokeBasin;
use snailfish::Snailfish;
use sonar_sweep::SonarSweep;
use syntax_scoring::SyntaxScoring;
use the_treacher_of_whales::TheTreacherOfWhales;
use transparent_origami::TransparentOrigami;
use trench_map::TrenchMap;
use trick_shot::TrickShot;

use std::error::Error;

use aoc_framework::{check_solved_tasks, BoxedAocTask};
type BoxedError = Box<dyn Error + Send + Sync>;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let tasks: Vec<BoxedAocTask> = vec![
        Box::new(RockPaperScissors),
        Box::new(SonarSweep),
        Box::new(Dive),
        Box::new(BinaryDiagnostic),
        Box::new(GiantSquid),
        Box::new(HydrothermalVenture),
        Box::new(Lanternfish),
        Box::new(TheTreacherOfWhales),
        Box::new(SevenSegmentSearch),
        Box::new(SmokeBasin),
        Box::new(SyntaxScoring),
        Box::new(DumboOctopus),
        Box::new(PassagePathing),
        Box::new(TransparentOrigami),
        Box::new(ExtendedPolymerization),
        Box::new(Chiton),
        Box::new(PacketDecoder),
        Box::new(TrickShot),
        Box::new(Snailfish),
        Box::new(BeaconScanner),
        Box::new(TrenchMap),
        Box::new(DiracDice),
        Box::new(ReactorReboot),
        Box::new(Amphipod),
        Box::new(ArithmeticLogicUnit),
        Box::new(SeaCucumber),
    ];

    check_solved_tasks(tasks, 2)?;

    Ok(())
}
