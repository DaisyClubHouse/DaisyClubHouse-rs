use std::{net::TcpListener, thread::spawn};

use tungstenite::accept;

const LISTENING_ADDR: &str = "127.0.0.1:9000";

fn main() {
    let server = TcpListener::bind(LISTENING_ADDR).unwrap();

    println!("监听中: {}", LISTENING_ADDR);

    for stream in server.incoming() {
        spawn(move || {
            let mut ws = accept(stream.unwrap()).unwrap();
            println!("Got a connection");
            loop {
                let msg = ws.read_message().unwrap();

                println!("[receive] {}", msg.to_string());

                if msg.is_binary() || msg.is_text() {
                    ws.write_message(msg).unwrap();
                }
            }
        });
    }
}
