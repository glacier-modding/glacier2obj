use std::{fs, io};
use std::fs::OpenOptions;
use std::io::prelude::*;

use tungstenite::{connect, Message};
use url::Url;

pub struct GameConnection;
impl GameConnection {
    pub fn get_brick_hashes_from_game() -> Vec<String> {
        println!("Connecting to EditorServer on port 46735...");
        io::stdout().flush().unwrap();

        let mut socket = GameConnection::connect_to_game();
        
        GameConnection::send_hello_message(&mut socket);
        GameConnection::send_message(&mut socket, "{\"type\":\"rebuildEntityTree\"}".to_string());
        GameConnection::send_message(&mut socket, "{\"type\":\"getBrickHashes\"}".to_string());
        return GameConnection::receive_brick_hashes_from_game(socket);
    }

    fn receive_brick_hashes_from_game(mut socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>) -> Vec<String> {
        let mut welcome_received: bool = false;
        let mut brick_messages: Vec<String> = Vec::new();
        loop {
            let msg = socket.read_message().expect("Error reading message");
            if msg.to_string().as_str() == "Done sending brick hashes." {
                println!("Received scene hash. Closing connection to EditorServer and returning scene hash.");
                io::stdout().flush().unwrap();
                break
            }
            if msg.to_string().as_str() == "{\"type\":\"entityTreeRebuilt\"}" {
                println!("Entity Tree rebuilt. Sending prims to game...");
                io::stdout().flush().unwrap();
                continue
            }
            if welcome_received {
                println!("Received scene hash from EditorServer: {}", msg);
                io::stdout().flush().unwrap();
                brick_messages.push(msg.to_string());
            } else {
                println!("Connected to EditorServer.");
                println!("Rebuilding entity tree...");
                io::stdout().flush().unwrap();
                welcome_received = true;
            }
        }
        return brick_messages;
    }

    pub fn get_entity_list_from_game(prims_file_path: &str, pf_boxes_file_path: &str) {
        println!("Connecting to EditorServer on port 46735...");
        io::stdout().flush().unwrap();

        let mut socket = GameConnection::connect_to_game();
        
        GameConnection::send_hello_message(&mut socket);

        GameConnection::send_message(&mut socket, r#"{"type":"listPrimEntities", "prims":[]}"#.to_string());
        GameConnection::send_message(&mut socket, r#"{"type":"listPfBoxEntities"}"#.to_string());
        
        GameConnection::clear_file(prims_file_path);
        GameConnection::clear_file(pf_boxes_file_path);
        
        let prims_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(prims_file_path)
            .unwrap();
        
        let pf_boxes_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(pf_boxes_file_path)
            .unwrap();
    

            GameConnection::build_entity_list(socket, prims_file, pf_boxes_file);
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

    fn get_input_file_contents(in_file_path: &str) -> String {
        println!("Loading toFind.json file...");
        io::stdout().flush().unwrap();
        let in_file_contents = fs::read_to_string(in_file_path).expect(format!("Error opening {}. Run a scan to generate this file", in_file_path).as_str());
        in_file_contents
    }

    fn send_hello_message(socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>) {
        println!("Sending hello message...");
        io::stdout().flush().unwrap();
        let _ = socket.write_message(Message::Text(r#"{"type":"hello","identifier":"glacier2obj"}"#.into()));
    }

    fn build_entity_list(mut socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, mut prims_file: fs::File, mut pf_boxes_file: fs::File) {
        let mut welcome_received: bool = false;
        let mut is_first: bool = true;
        let mut reading_prims: bool = true;
        
        loop {
            let msg = socket.read_message().expect("Error reading message");
            if msg.to_string().as_str() == "Done sending entities." {
                if reading_prims {
                    reading_prims = false;
                    println!("Received Done message for prims. Finalizing prims.json output and getting pf boxes.");
                    io::stdout().flush().unwrap();
                    if let Err(e) = writeln!(prims_file, "]}}") {
                        eprintln!("Couldn't write to prims file: {}", e);
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
                println!("Entity Tree rebuilt. Sending prims to game...");
                io::stdout().flush().unwrap();
                continue
            }
            if welcome_received {
                if !is_first {
                    if reading_prims {
                        if let Err(e) = writeln!(prims_file, ",") {
                            eprintln!("Couldn't write to prims file: {}", e);
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
                    if reading_prims {
                        println!("Received first PRIM transform from EditorServer. Continuing to process PRIM transforms...");
                        io::stdout().flush().unwrap();
                    } else {
                        println!("Received first pf box transform from EditorServer. Continuing to process pf box transforms...");
                        io::stdout().flush().unwrap();
                    }
                }
                if reading_prims { 
                    if let Err(e) = write!(prims_file, "{}", msg) {
                        eprintln!("Couldn't write to prims file: {}", e);
                    }
                }
                else { 
                    if let Err(e) = write!(pf_boxes_file, "{}", msg) {
                        eprintln!("Couldn't write to pf boxes file: {}", e);
                    }
                }
            } else {
                if let Err(e) = write!(prims_file, r#"{{"entities":["#) {
                    eprintln!("Couldn't write to prims file: {}", e);
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