use std::{fs, io};
use std::fs::OpenOptions;
use std::io::prelude::*;

use tungstenite::{connect, Message};
use url::Url;

pub struct GameConnection;
impl GameConnection {
    pub fn get_prim_list_from_game(in_file_path: &str, out_file_path: &str) {
        println!("Connecting to EditorServer on port 46735...");
        io::stdout().flush().unwrap();

        let mut socket = GameConnection::connect_to_game();
        
        GameConnection::send_hello_message(&mut socket);
        let in_file_contents = GameConnection::get_input_file_contents(in_file_path);

        GameConnection::send_message(&mut socket, in_file_contents);
        
        GameConnection::clear_file(out_file_path);
        
        let out_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(out_file_path)
            .unwrap();

            GameConnection::build_prims_list(socket, out_file);
    }

    fn connect_to_game() -> tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>> {
        let (socket, _response) = connect(
            Url::parse("ws://localhost:46735/socket").unwrap()
        ).expect("Can't connect");
        socket
    }

    fn clear_file(out_file_path: &str) {
        fs::write(out_file_path, "").expect(format!("Error writing to {}", out_file_path).as_str());
    }

    fn send_message(socket: &mut tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, in_file_contents: String) {
        println!("Sending prims to game...");
        io::stdout().flush().unwrap();
        let _ = socket.write_message(Message::Text(in_file_contents.into()));
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

    fn build_prims_list(mut socket: tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>, mut out_file: fs::File) {
        let mut welcome_received: bool = false;
        let mut is_first: bool = true;
        
        loop {
            let msg = socket.read_message().expect("Error reading message");
            if msg.to_string().as_str() == "Done sending entities." {
                println!("Received Done message. Finalizing json output and exiting.");
                io::stdout().flush().unwrap();
                if let Err(e) = writeln!(out_file, "]}}") {
                    eprintln!("Couldn't write to file: {}", e);
                    io::stdout().flush().unwrap();
                }
                break
            }
            if welcome_received {
                if !is_first {
                    if let Err(e) = writeln!(out_file, ",") {
                        eprintln!("Couldn't write to file: {}", e);
                        io::stdout().flush().unwrap();
                    }
                } else {
                    println!("Received first PRIM transform from EditorServer. Continuing to process PRIM transforms...");
                    io::stdout().flush().unwrap();
                    is_first = false;
                }
                if let Err(e) = write!(out_file, "{}", msg) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            } else {
                if let Err(e) = write!(out_file, r#"{{"entities":["#) {
                    eprintln!("Couldn't write to file: {}", e);
                }
                println!("Connected to EditorServer.");
                io::stdout().flush().unwrap();
                welcome_received = true;
            }
        }
    }
}