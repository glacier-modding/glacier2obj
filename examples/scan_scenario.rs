use std::env;

use glacier2obj::scanner::scenario_scan::ScenarioScan;

// Based on mount_game_files example from rpkg-rs
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        eprintln!("Usage: cargo run -- example scan_scenario <path to a Retail directory> <game version (H2016 | HM2 | HM3)> <ioi string or hash> <path to a hashlist> <path to output file>");
        return;
    }
    let mut scan: ScenarioScan = ScenarioScan::new(args[1].clone(), args[2].clone(), args[3].clone(), args[4].clone());
    scan.scan_scenario();
    scan.output_to_file(args[5].clone());
    return;
}
