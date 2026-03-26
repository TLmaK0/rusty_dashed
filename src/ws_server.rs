use std::sync::mpsc::{channel, Sender};
use std::sync::Mutex;
use std::thread::spawn;
use ws::{Handler as WsHandler, Handshake, Result as WsResult};

pub struct WsServer;

lazy_static! {
    static ref SENDER: Mutex<Sender<String>> = Mutex::new(WsServer::start_ws());
}

struct WsHandler_;

impl WsHandler for WsHandler_ {
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        Ok(())
    }
}

impl WsServer {
    fn start_ws() -> Sender<String> {
        let web_socket = ws::WebSocket::new(|_| WsHandler_).unwrap();
        let broadcast = web_socket.broadcaster();

        spawn(move || {
            web_socket.listen("0.0.0.0:3001").unwrap();
        });

        let (tx, rx) = channel();

        spawn(move || loop {
            let message = rx.recv().unwrap();
            let _ = broadcast.send(message);
        });
        tx
    }

    pub fn send_message(message: String) {
        SENDER.lock().unwrap().clone().send(message).unwrap();
    }
}
