use std::{env, io::{self, Write}};

use glacier2obj::{connect::game_connection::GameConnection, extract::aloc_extraction::AlocExtraction, json_serde::entities_json::EntitiesJson, package::package_scan::PackageScan};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        eprintln!("Usage: cargo run <path to a Retail directory> <game version (H2016 | HM2 | HM3)> <path to alocs.json file> <path to pfBoxes.json file> <path to a Runtime directory> <path to output prim directory>");
        return;
    }

    io::stdout().flush().unwrap();
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[2].clone()).unwrap();

    GameConnection::get_entity_list_from_game(args[3].as_str(), args[4].as_str());
    let alocs_json = EntitiesJson::build_from_alocs_file(args[3].clone());
    let needed_aloc_hashes = AlocExtraction::get_all_aloc_hashes(&alocs_json, args[6].clone());

    if needed_aloc_hashes.is_empty() {
        println!("All aloc files already exist. Skipping extraction.");
        io::stdout().flush().unwrap();

    } else {
        println!("Extracting {} alocs.", needed_aloc_hashes.len());
        io::stdout().flush().unwrap();

        AlocExtraction::extract_alocs(args[5].clone(), needed_aloc_hashes, &partition_manager, args[6].clone());
    }
    println!("Done building alocs.json and extracting alocs from scenario.");
    io::stdout().flush().unwrap();

}
