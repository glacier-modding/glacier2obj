use std::{env, fs};

use tungstenite::{connect, Message};
use url::Url;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- example connect_to_game <port>");
        return;
    }
    println!("Connecting to game on port {}", args[1]);

    let (mut socket, _response) = connect(
        Url::parse(("ws://localhost:".to_string() + &args[1] + "/socket").as_str()).unwrap()
    ).expect("Can't connect");
    
    let _ = socket.write_message(Message::Text(r#"{"type":"hello","identifier":"glacier2obj"}"#.into()));
    // let message = r#"{"type":"listTbluEntities", "tblus":["002E141E1B1C6EFE","006987D492265FBC","00C7E348A80A6E6E","006734FBA4F1E68D","0055D313795CFA48","008A2B97E3510758","00B6E9D9DA6DE143","00F032EB6C229123"]}"#;
    let message = fs::read_to_string("toFind.json").expect("Error opening toFind.json. Run a scan to generate this file");

    let _ = socket.write_message(Message::Text(message.into()));
    
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
