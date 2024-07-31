use std::{env, io::{self, Write}};

use glacier2obj::{connect::game_connection::GameConnection, extract::prim_extraction::PrimExtraction, json_serde::entities_json::EntitiesJson, package::package_scan::PackageScan};

// Based on mount_game_files example from rpkg-rs
pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        eprintln!("Usage: cargo run <path to a Retail directory> <game version (H2016 | HM2 | HM3)> <path to prims.json file> <path to pfBoxes.json file> <path to a Runtime directory> <path to output prim directory>");
        return;
    }
    let brick_tblu_hashes: Vec<String> = EntitiesJson::get_brick_tblu_hashes(GameConnection::get_brick_hashes_from_game());
    println!("Scene tblu hashes: {:?}", brick_tblu_hashes);
    io::stdout().flush().unwrap();
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[2].clone()).unwrap();

    GameConnection::get_entity_list_from_game(args[3].as_str(), args[4].as_str());
    let prims_json = EntitiesJson::build_from_prims_file(args[3].clone());
    let needed_prim_hashes = PrimExtraction::get_needed_prim_hashes(&prims_json, args[6].clone());
    if needed_prim_hashes.is_empty() {
        println!("All prim files already exist. Skipping extraction.");
        io::stdout().flush().unwrap();

    } else {
        println!("Extracting {} prims.", needed_prim_hashes.len());
        io::stdout().flush().unwrap();

        PrimExtraction::extract_prims(args[5].clone(), needed_prim_hashes, &partition_manager, args[6].clone());
    }
    println!("Done building prims.json and extracting prims from scenario.");
    io::stdout().flush().unwrap();

}
