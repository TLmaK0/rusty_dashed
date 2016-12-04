use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Mutex;
use std::thread::{spawn, JoinHandle};
use ws::{Handler as WsHandler, Handshake, Result as WsResult, Sender as WsSender, listen, Error, Message, WebSocket};

pub struct WsServer {
    client: WsSender
}

lazy_static! {
    static ref SENDER: Mutex<Sender<String>> = Mutex::new(WsServer::start_ws());
}

impl WsServer {
    fn start_ws() -> Sender<String> {
        let mut webSocket = WebSocket::new(|out| {
            WsServer{client: out}
        }).unwrap();
        let broadcast = webSocket.broadcaster();

        spawn(move || {
            webSocket.listen("0.0.0.0:3001").unwrap();
        });

        let (tx, rx) = channel();

        spawn(move || {
            loop {
                let message = rx.recv().unwrap();
                broadcast.send(message);
            }
        });
        tx
    }

    pub fn send_message(message: String){
        //TODO: remove this lock
        SENDER.lock().unwrap().clone().send(message);
    }
}

impl WsHandler for WsServer {
}


