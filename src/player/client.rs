use std::net::TcpStream;
use tungstenite::WebSocket;

pub struct PlayerClient {
    ws: WebSocket<TcpStream>,
}

impl PlayerClient {
    pub fn new(ws: WebSocket<TcpStream>) -> PlayerClient {
        PlayerClient { ws }
    }

    pub fn run(&mut self) {
        loop {
            // TODO check if connection is avaible
            match self.ws.read_message() {
                Ok(msg) => {
                    println!("[receive] {}", msg.to_string());

                    if msg.is_binary() || msg.is_text() {
                        self.ws.write_message(msg).unwrap();
                    }
                }
                Err(e) => {
                    println!("[error] {}", e);
                    break;
                }
            };
        }
    }
}
