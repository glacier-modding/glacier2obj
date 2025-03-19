use glacier2obj::extract::aloc_extraction::AlocExtraction;
use glacier2obj::json_serde::entities_json::EntitiesJson;
use glacier2obj::package::package_scan::PackageScan;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 6 {
        eprintln!("Usage: cargo run --example extract_alocs -- <path to a Retail directory> <path to a Runtime directory> <game version (H2016 | HM2 | HM3)> <path to a alocs.json file> <path to output directory>");
        return;
    }
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[3].clone()).unwrap();
    let alocs_json = EntitiesJson::build_from_nav_json_file(args[4].clone());
    let needed_aloc_hashes = AlocExtraction::get_all_aloc_hashes(&alocs_json, args[5].clone());
    if needed_aloc_hashes.is_empty() {
        println!("All aloc files already exist. Skipping extraction.");
        return;
    }
    println!("Extracting {} alocs.", needed_aloc_hashes.len());
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[3].clone()).unwrap();
    AlocExtraction::extract_alocs(args[2].clone(), needed_aloc_hashes, &partition_manager, args[5].clone());
}