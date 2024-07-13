use std::env;

use glacier2obj::{package::package_scan::PackageScan, scene::scene_scan::SceneScan};

// Based on mount_game_files example from rpkg-rs
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        eprintln!("Usage: cargo run -- example scan_scenario <path to a Retail directory> <game version (H2016 | HM2 | HM3)> <ioi string or hash> <path to a hashlist> <path to output file>");
        return;
    }
    let mut scan: SceneScan = SceneScan::new(vec![args[3].clone()], args[4].clone());
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[2].clone()).unwrap();

    scan.scan_scenario(&partition_manager);
    scan.output_to_file(args[5].clone());
}
