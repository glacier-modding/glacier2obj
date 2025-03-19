use glacier2obj::json_serde::entities_json::EntitiesJson;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example read_alocs -- <path to a alocs.json file>");
        return;
    }
    let mut alocs_json: EntitiesJson = EntitiesJson::build_from_nav_json_file(args[1].clone());
    alocs_json.output_entities();
}