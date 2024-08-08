use std::{fs, io};
use std::fs::OpenOptions;
use std::io::prelude::*;

use tungstenite::{connect, Message};
use url::Url;

pub struct GameConnection;
impl GameConnection {
    pub fn get_entity_list_from_game(alocs_file_path: &str, pf_boxes_file_path: &str) {
        println!("Connecting to EditorServer on port 46735...");
        io::stdout().flush().unwrap();

        let mut socket = GameConnection::connect_to_game();
        
        GameConnection::send_hello_message(&mut socket);
        GameConnection::send_message(&mut socket, "{\"type\":\"rebuildEntityTree\"}".to_string());
        GameConnection::send_message(&mut socket, r#"{"type":"listAlocEntities"}"#.to_string());
        GameConnection::send_message(&mut socket, r#"{"type":"listPfBoxEntities"}"#.to_string());
        
        GameConnection::clear_file(alocs_file_path);
        GameConnection::clear_file(pf_boxes_file_path);
        
        let alocs_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(alocs_file_path)
            .unwrap();
        
        let pf_boxes_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(pf_boxes_file_path)
            .unwrap();
    

            GameConnection::build_entity_list(socket, alocs_file, pf_boxes_file);
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

    fn build_entity_list(mut socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, mut alocs_file: fs::File, mut pf_boxes_file: fs::File) {
        let mut welcome_received: bool = false;
        let mut is_first: bool = true;
        let mut reading_alocs: bool = true;
        
        loop {
            let msg = socket.read_message().expect("Error reading message");
            if msg.to_string().as_str() == "Done sending entities." {
                if reading_alocs {
                    reading_alocs = false;
                    println!("Received Done message for alocs. Finalizing alocs.json output and getting pf boxes.");
                    io::stdout().flush().unwrap();
                    if let Err(e) = writeln!(alocs_file, "]}}") {
                        eprintln!("Couldn't write to alocs file: {}", e);
                        io::stdout().flush().unwrap();
                    }
                    is_first = true;
                    continue;
                } else {
                    println!("Received Done message for pf boxes. Finalizing pfBoxes.json output and exiting.");
                    io::stdout().flush().unwrap();
                    if let Err(e) = writeln!(pf_boxes_file, "]}}") {
                        eprintln!("Couldn't write to pf boxes file: {}", e);
                        io::stdout().flush().unwrap();
                    }
                    break
                }
            }
            if msg.to_string().as_str() == "{\"type\":\"entityTreeRebuilt\"}" {
                println!("Entity Tree rebuilt.");
                io::stdout().flush().unwrap();
                continue
            }
            if welcome_received {
                if !is_first {
                    if reading_alocs {
                        if let Err(e) = writeln!(alocs_file, ",") {
                            eprintln!("Couldn't write to alocs file: {}", e);
                            io::stdout().flush().unwrap();
                        }
                    } else {
                        if let Err(e) = writeln!(pf_boxes_file, ",") {
                            eprintln!("Couldn't write to pf boxes file: {}", e);
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
                    if let Err(e) = write!(alocs_file, "{}", msg) {
                        eprintln!("Couldn't write to alocs file: {}", e);
                    }
                }
                else { 
                    if let Err(e) = write!(pf_boxes_file, "{}", msg) {
                        eprintln!("Couldn't write to pf boxes file: {}", e);
                    }
                }
            } else {
                if let Err(e) = write!(alocs_file, r#"{{"entities":["#) {
                    eprintln!("Couldn't write to alocs file: {}", e);
                }
                if let Err(e) = write!(pf_boxes_file, r#"{{"entities":["#) {
                    eprintln!("Couldn't write to pf boxes file: {}", e);
                }
                println!("Connected to EditorServer.");
                io::stdout().flush().unwrap();
                welcome_received = true;
            }
        }
    }
}