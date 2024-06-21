use std::env;

use glacier2obj::{connect::game_connection::GameConnection, extract::prim_extraction::PrimExtraction, json_serde::prims_json::PrimsJson, package::package_scan::PackageScan, scenario::scenario_scan::ScenarioScan};

// Based on mount_game_files example from rpkg-rs
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 6 {
        eprintln!("Usage: cargo run -- example export_scenario <path to a Retail directory> <game version (H2016 | HM2 | HM3)> <scenario ioi string or hash> <path to a hashlist> <path to toFind file> <path to prims.json file> <path to a Runtime directory> <path to output prims directory>");
        return;
    }
    let mut scan: ScenarioScan = ScenarioScan::new(args[3].clone(), args[4].clone());
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[2].clone()).unwrap();

    scan.scan_scenario(&partition_manager);
    scan.output_to_file(args[5].clone());

    GameConnection::get_prim_list_from_game(args[5].as_str(), args[6].as_str());
    let prims_json = PrimsJson::build_from_prims_file(args[6].clone());
    let needed_prim_hashes = PrimExtraction::get_needed_prim_hashes(&prims_json, args[8].clone());
    if needed_prim_hashes.is_empty() {
        println!("All prim files already exist. Skipping extraction.");
    } else {
        println!("Extracting {} prims.", needed_prim_hashes.len());
        PrimExtraction::extract_prims(args[7].clone(), needed_prim_hashes, &partition_manager, args[8].clone());
    }
    println!("Done building prims.json and extracting prims from scenario.")
}
