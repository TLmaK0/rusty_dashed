#[macro_use] extern crate log;
extern crate env_logger;
extern crate rusty_dashed;
extern crate ws;
use rusty_dashed::Dashboard;
use std::thread::spawn;
use std::option::Option;
use ws::{Handler as WsHandler, Handshake, Result as WsResult, Sender as WsSender, listen, Error, Message, WebSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use std::io;
use std::io::Write;
struct MySender{
    sender: Option<WsSender>
}

fn main() {
    env_logger::init().unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    let tx1 = tx.clone();
        let data1 = r#"a1({
          "nodes": [
            {"id": "Myriel", "group": 1},
            {"id": "Napoleon", "group": 1},
            {"id": "Mlle.Baptistine", "group": 1}
          ],
          "links": [
            {"source": "Napoleon", "target": "Myriel", "value": 1},
            {"source": "Mlle.Baptistine", "target": "Myriel", "value": 8},
            {"source": "Napoleon", "target": "Mlle.Baptistine", "value": 4}
          ]
        })"#;

    let server = spawn(move || {
        let mut dashboard = Dashboard::new();
        dashboard.add_graph("a1", "test1", 0, 0, 4, 4);
        dashboard.add_graph("a2", "test2", 4, 0, 4, 4);
        dashboard.add_graph("a3", "test1", 8, 0, 4, 4);
        dashboard.add_graph("a4", "test2", 0, 4, 4, 4);
        let rusty_dashed = rusty_dashed::Server::new(dashboard);
        let sender = rusty_dashed.start();
        loop {
            let message = rx.recv().unwrap();
            sender.send(message);
        }
    });
    std::thread::sleep_ms(10000);
    tx1.send(data1).unwrap();
    server.join();
}
