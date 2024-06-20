use glacier2obj::extract::prim_extraction::PrimExtraction;
use glacier2obj::json_serde::prims_json::PrimsJson;
use glacier2obj::package::package_scan::PackageScan;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 6 {
        eprintln!("Usage: cargo run --example extract_prims -- <path to a Retail directory> <path to a Runtime directory> <game version (H2016 | HM2 | HM3)> <path to a prims.json file> <path to output directory>");
        return;
    }
    let prims_json = PrimsJson::build_from_prims_file(args[4].clone());
    let needed_prim_hashes = PrimExtraction::get_needed_prim_hashes(&prims_json, args[5].clone());
    if needed_prim_hashes.is_empty() {
        println!("All prim files already exist. Skipping extraction.");
        return;
    }
    println!("Extracting {} prims.", needed_prim_hashes.len());
    let partition_manager = PackageScan::scan_packages(args[1].clone(), args[3].clone()).unwrap();
    PrimExtraction::extract_prims(args[2].clone(), needed_prim_hashes, &partition_manager, args[5].clone());
}