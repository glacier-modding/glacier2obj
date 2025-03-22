use std::{env, io::{self, Write}};

use glacier2obj::{connect::game_connection::GameConnection, extract::aloc_extraction::AlocExtraction, package::package_scan::PackageScan};
use glacier2obj::json_serde::entities_json::{EntitiesJson};

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!("Usage: cargo run <path to a Retail directory> <game version (H2016 | HM2 | HM3)> <path to nav.json file> <path to a Runtime directory> <path to output aloc directory>");
        return;
    }

    io::stdout().flush().unwrap();
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[2].clone()).unwrap();

    GameConnection::get_entity_list_from_game(args[3].as_str());
    let nav_json = EntitiesJson::build_from_nav_json_file(args[3].clone());
    let needed_aloc_hashes = AlocExtraction::get_all_aloc_hashes(&nav_json, args[5].clone());

    if needed_aloc_hashes.is_empty() {
        println!("All aloc files already exist. Skipping extraction.");
        io::stdout().flush().unwrap();
    } else {
        println!("Extracting {} alocs.", needed_aloc_hashes.len());
        io::stdout().flush().unwrap();

        AlocExtraction::extract_alocs(args[4].clone(), needed_aloc_hashes, &partition_manager, args[5].clone());
    }
    println!("Done building extracting alocs, pf boxes, and pf seed points from scenario and building output.nav.json.");
    io::stdout().flush().unwrap();
}
