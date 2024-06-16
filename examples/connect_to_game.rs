use std::{env, fs};
use std::fs::OpenOptions;
use std::io::prelude::*;

use tungstenite::{connect, Message};
use url::Url;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- example connect_to_game <in_file> <out_file>");
        return;
    }
    println!("Connecting to game on port 46735");

    let (mut socket, _response) = connect(
        Url::parse("ws://localhost:46735/socket").unwrap()
    ).expect("Can't connect");
    
    let _ = socket.write_message(Message::Text(r#"{"type":"hello","identifier":"glacier2obj"}"#.into()));
    let message = fs::read_to_string(args[1].as_str()).expect(format!("Error opening {}. Run a scan to generate this file", args[1].as_str()).as_str());

    let _ = socket.write_message(Message::Text(message.into()));
    
    fs::write(args[2].as_str(), "").expect(format!("Error writing to {}", args[2].as_str()).as_str());
    
    let mut out_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(args[2].as_str())
        .unwrap();

    let mut welcome_received: bool = false;
    let mut is_first: bool = true;

    loop {
        let msg = socket.read_message().expect("Error reading message");
        if msg.to_string().as_str() == "Done sending entities." {
            println!("Received Done message. Finalizing json output and exiting.");
            if let Err(e) = writeln!(out_file, "]}}") {
                eprintln!("Couldn't write to file: {}", e);
            }
            break
        }
        if welcome_received {
            if !is_first {
                if let Err(e) = writeln!(out_file, ",") {
                    eprintln!("Couldn't write to file: {}", e);
                }
            } else {
                is_first = false;
            }
            if let Err(e) = write!(out_file, "{}", msg) {
                eprintln!("Couldn't write to file: {}", e);
            }
        } else {
            if let Err(e) = write!(out_file, r#"{{"entities":["#) {
                eprintln!("Couldn't write to file: {}", e);
            }
            welcome_received = true;
        }
    }
}
