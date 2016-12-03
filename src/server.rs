use iron::headers::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use std::fs::File;
use std::io::prelude::*;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use mount::Mount;
use ws::{Handler as WsHandler, Handshake, Result as WsResult, Sender as WsSender, listen, Error, Message};
use std::path::Path;
use std::thread::{spawn, JoinHandle};
use std::result::Result;
use iron::middleware::Handler;
use std::sync::mpsc::{Sender, Receiver, channel};
use Dashboard;

struct WsServer {
    client: WsSender
}

impl WsHandler for WsServer {
    fn on_message(&mut self, msg: Message) -> WsResult<()> {
        self.client.send(msg)
    }
    
    fn on_open(&mut self, _: Handshake) -> WsResult<()> {
        //connection in        
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

        let data2 = r#"a3({
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
        let data3 = r#"a1({
          "nodes": [
            {"id": "Myriel", "group": 1},
            {"id": "Napoleon", "group": 1},
            {"id": "Mlle.Baptistine", "group": 1}
          ],
          "links": [
            {"source": "Mlle.Baptistine", "target": "Myriel", "value": 8},
            {"source": "Napoleon", "target": "Mlle.Baptistine", "value": 4}
          ]
        })"#;
        self.client.send(data1);
        self.client.send(data2);
        self.client.send(data3)
    }
}

include!(concat!(env!("OUT_DIR"), "/public.rs"));

pub struct Server {
    pub dashboard: Dashboard,
    channel: (Sender<String>, Receiver<String>)
}

impl Server {
    pub fn new(dashboard: Dashboard) -> Server {
        Server { dashboard: dashboard, channel: channel() }
    }

    fn get_static_file(req: &mut Request) -> IronResult<Response> {
        let request_path = format!("public/{}", req.url.path().join("/"));
        let file_path = match request_path.as_ref() {
            "public/" => "public/index.html",
            path => path
        };

        let content = Server::get_file_content(&file_path); 

        let content_type = match Path::new(&file_path).extension().unwrap().to_str().unwrap() {
            "html" => ContentType::html(),
            "css" => ContentType(Mime(TopLevel::Text, SubLevel::Css, vec![])),
            "js" | "json" => ContentType(Mime(TopLevel::Application, SubLevel::Javascript, vec![])),
            _ => unimplemented!()
        };

        let response = Response::with((status::Ok, content, Header(content_type)));
        Ok(response)
    }

    #[cfg(feature = "serve_static")]
    fn get_file_content(file_path: &str) -> String {
        PUBLIC.get(&file_path).map(
                |file_content| std::str::from_utf8(&file_content).unwrap().to_owned()
            ).unwrap_or(String::from(""))
    }

    #[cfg(feature = "debug_static")]
    fn get_file_content(file_path: &str) -> String {
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }


    pub fn send(message: &str) {

    }

    pub fn start(&self) {
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();

        let dashboard = DashboardMount{dashboard: self.dashboard.get_init_script().to_owned()};
        spawn(move || {
            let mut mount = Mount::new();
            mount.mount("/", Server::get_static_file)
                .mount("/js/rusty-dashed.js", dashboard);
            Iron::new(mount).http("0.0.0.0:3000").unwrap();
        }); 

        listen("0.0.0.0:3001", |client| {
            WsServer { client: client }
        }).unwrap()
    }
}

struct DashboardMount {
    pub dashboard: String
}

impl Handler for DashboardMount {
    fn handle(&self, _req: &mut Request) -> IronResult<Response>{
        Ok(Response::with((status::Ok, self.dashboard.to_owned())))
    }
}

