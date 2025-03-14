use std::{fs, io};
use std::fs::OpenOptions;
use std::io::prelude::*;

use tungstenite::{connect, Message};
use url::Url;

pub struct GameConnection;
impl GameConnection {
    pub fn get_entity_list_from_game(navkit_json_file_path: &str) {
        println!("Connecting to EditorServer on port 46735...");
        io::stdout().flush().unwrap();

        let mut socket = GameConnection::connect_to_game();

        GameConnection::send_hello_message(&mut socket);
        GameConnection::send_message(&mut socket, "{\"type\":\"rebuildEntityTree\"}".to_string());
        GameConnection::send_message(&mut socket, r#"{"type":"listAlocEntities"}"#.to_string());
        GameConnection::send_message(&mut socket, r#"{"type":"listPfBoxEntities"}"#.to_string());

        GameConnection::clear_file(navkit_json_file_path);

        let nav_json_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(navkit_json_file_path)
            .unwrap();

        GameConnection::build_entity_list(socket, nav_json_file);
    }

    fn connect_to_game() -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
        let (socket, _response) = connect(
            Url::parse("ws://localhost:46735/socket").unwrap()
        ).expect("Can't connect");
        socket
    }

    fn clear_file(file_path: &str) {
        fs::write(file_path, "").expect(format!("Error writing to {}", file_path).as_str());
    }

    fn send_message(socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, message: String) {
        io::stdout().flush().unwrap();
        let _ = socket.write_message(Message::Text(message.into()));
    }

    fn send_hello_message(socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>) {
        println!("Sending hello message...");
        io::stdout().flush().unwrap();
        let _ = socket.write_message(Message::Text(r#"{"type":"hello","identifier":"glacier2obj"}"#.into()));
    }

    fn build_entity_list(mut socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, mut nav_json_file: fs::File) {
        let mut welcome_received: bool = false;
        let mut is_first: bool = true;
        let mut reading_alocs: bool = true;

        loop {
            let msg = socket.read_message().expect("Error reading message");
            if msg.to_string().as_str() == "Done sending entities." {
                if reading_alocs {
                    reading_alocs = false;
                    println!("Received Done message for alocs. Getting pf boxes.");
                    io::stdout().flush().unwrap();
                    if let Err(e) = writeln!(nav_json_file, r#"],"pfBoxes":["#) {
                        eprintln!("Couldn't write to alocs file: {}", e);
                        io::stdout().flush().unwrap();
                    }
                    is_first = true;
                    continue;
                } else {
                    println!("Received Done message for pf boxes. Finalizing output.navkit.json output and exiting.");
                    io::stdout().flush().unwrap();
                    if let Err(e) = writeln!(nav_json_file, "]}}") {
                        eprintln!("Couldn't write to pf boxes file: {}", e);
                        io::stdout().flush().unwrap();
                    }
                    break;
                }
            }
            if msg.to_string().as_str() == "{\"type\":\"entityTreeRebuilt\"}" {
                println!("Entity Tree rebuilt.");
                io::stdout().flush().unwrap();
                continue;
            }
            if welcome_received {
                if !is_first {
                    if reading_alocs {
                        if let Err(e) = writeln!(nav_json_file, ",") {
                            eprintln!("Couldn't write to nav json file: {}", e);
                            io::stdout().flush().unwrap();
                        }
                    } else {
                        if let Err(e) = writeln!(nav_json_file, ",") {
                            eprintln!("Couldn't write to nav json file: {}", e);
                            io::stdout().flush().unwrap();
                        }
                    }
                } else {
                    is_first = false;
                    if reading_alocs {
                        println!("Received first ALOC transform from EditorServer. Continuing to process ALOC transforms...");
                        io::stdout().flush().unwrap();
                    } else {
                        println!("Received first pf box transform from EditorServer. Continuing to process pf box transforms...");
                        io::stdout().flush().unwrap();
                    }
                }
                if reading_alocs {
                    if let Err(e) = write!(nav_json_file, "{}", msg) {
                        eprintln!("Couldn't write to nav json file: {}", e);
                    }
                } else {
                    if let Err(e) = write!(nav_json_file, "{}", msg) {
                        eprintln!("Couldn't write to nav json file: {}", e);
                    }
                }
            } else {
                if let Err(e) = write!(nav_json_file, r#"{{"alocs":["#) {
                    eprintln!("Couldn't write to nav json file: {}", e);
                }
                println!("Connected to EditorServer.");
                io::stdout().flush().unwrap();
                welcome_received = true;
            }
        }
    }
}