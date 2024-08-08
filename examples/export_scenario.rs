use std::env;

use glacier2obj::{connect::game_connection::GameConnection, extract::aloc_extraction::AlocExtraction, json_serde::entities_json::EntitiesJson, package::package_scan::PackageScan};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        eprintln!("Usage: cargo run -- example export_scenario <path to a Retail directory> <game version (H2016 | HM2 | HM3)> <path to prims.json file> <path to pfBoxes.json file> <path to a Runtime directory> <path to output prims directory>");
        return;
    }
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[2].clone()).unwrap();

    GameConnection::get_entity_list_from_game(args[3].as_str(), args[4].as_str());
    let prims_json = EntitiesJson::build_from_alocs_file(args[3].clone());
    let needed_prim_hashes = AlocExtraction::get_all_aloc_hashes(&prims_json, args[6].clone());
    if needed_prim_hashes.is_empty() {
        println!("All prim files already exist. Skipping extraction.");
    } else {
        println!("Extracting {} prims.", needed_prim_hashes.len());
        AlocExtraction::extract_alocs(args[5].clone(), needed_prim_hashes, &partition_manager, args[6].clone());
    }
    println!("Done building prims.json and extracting prims from scenario.")
}
