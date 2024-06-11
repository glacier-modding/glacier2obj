use std::env;

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
    // let desk = "feeda4e80bc4b859";
    // let _ = socket.write_message(Message::Text(r#"{"type":"selectEntity","entity":{"id": "feede50ae0479660","tblu":"003FC5B5BE3EA0CE"}}"#.into()));
    let _ = socket.write_message(Message::Text(r#"{"type":"listEntities"}"#.into()));
    
    loop {
        let msg = socket.read_message().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
