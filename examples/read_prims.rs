use glacier2obj::json_serde::entities_json::EntitiesJson;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example read_prims -- <path to a prims.json file>");
        return;
    }
    let mut prims_json: EntitiesJson = EntitiesJson::build_from_alocs_file(args[1].clone());
    prims_json.output_entities();
}