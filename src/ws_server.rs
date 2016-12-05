use std::sync::mpsc::{Sender, channel};
use std::sync::Mutex;
use std::thread::{spawn};
use ws::{Handler as WsHandler, WebSocket};

pub struct WsServer {
}

lazy_static! {
    static ref SENDER: Mutex<Sender<String>> = Mutex::new(WsServer::start_ws());
}

impl WsServer {
    fn start_ws() -> Sender<String> {
        let web_socket = WebSocket::new(|_| {
            WsServer{}
        }).unwrap();
        let broadcast = web_socket.broadcaster();

        spawn(move || {
            web_socket.listen("0.0.0.0:3001").unwrap();
        });

        let (tx, rx) = channel();

        spawn(move || {
            loop {
                let message = rx.recv().unwrap();
                broadcast.send(message).unwrap();
            }
        });
        tx
    }

    pub fn send_message(message: String){
        //TODO: remove this lock
        SENDER.lock().unwrap().clone().send(message).unwrap();
    }
}

impl WsHandler for WsServer {
}


