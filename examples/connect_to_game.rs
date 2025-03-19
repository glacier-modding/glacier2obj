use std::env;

use glacier2obj::connect::game_connection::GameConnection;


pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- example connect_to_game <path to an output nav.json file>");
        return;
    }
    GameConnection::get_entity_list_from_game(args[1].as_str());
}