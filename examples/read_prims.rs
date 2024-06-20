use glacier2obj::json_serde::prims_json::PrimsJson;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example read_prims -- <path to a prims.json file>");
        return;
    }
    let mut prims_json: PrimsJson = PrimsJson::build_from_prims_file(args[1].clone());
    prims_json.output_prims();
}