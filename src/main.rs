use std::{net::TcpListener, thread::spawn};

use tungstenite::accept;

mod player;
// mod goband;

const LISTENING_ADDR: &str = "127.0.0.1:9000";

fn main() {
    let server = TcpListener::bind(LISTENING_ADDR).unwrap();

    println!("监听中: {}", LISTENING_ADDR);

    for stream in server.incoming() {
        spawn(move || {
            let ws = accept(stream.unwrap()).unwrap();

            player::client::PlayerClient::new(ws).run();
        });
    }
}
