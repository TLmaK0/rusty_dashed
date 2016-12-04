use iron::headers::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use std::fs::File;
use std::io::prelude::*;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use mount::Mount;
use ws::{Handler as WsHandler, Handshake, Result as WsResult, Sender as WsSender, listen, Error, Message, WebSocket};
use std::path::Path;
use std::thread::{spawn, JoinHandle};
use std::result::Result;
use iron::middleware::Handler;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::str::from_utf8;
use std::sync::Mutex;
use Dashboard;

struct WsServer {
    client: WsSender
}

impl WsHandler for WsServer {
}

include!(concat!(env!("OUT_DIR"), "/public.rs"));

pub struct Server {
    dashboard: Option<Dashboard>,
    sender: Option<Sender<String>>
}

lazy_static! {
    static ref SERVER: Mutex<Server> = Mutex::new(Server{ dashboard: None, sender: None });
}

impl Server {
    pub fn serve_dashboard(dashboard: Dashboard) -> JoinHandle<&'static str> {
        let mut server = SERVER.lock().unwrap();
        server.dashboard = Some(dashboard);
        let (sender, thread) = server.start();
        server.sender = Some(sender);
        thread
    }

    pub fn send_message(message: String){
        SERVER.lock().unwrap().sender.clone().unwrap().send(message);
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
                |file_content| from_utf8(&file_content).unwrap().to_owned()
            ).unwrap_or(String::from(""))
    }

    #[cfg(feature = "debug_static")]
    fn get_file_content(file_path: &str) -> String {
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }

    fn start(&self) -> (Sender<String>, JoinHandle<&'static str>) {
        let dashboard = DashboardMount{dashboard: self.dashboard.clone().unwrap().get_init_script().to_owned()};
        spawn(move || {
            let mut mount = Mount::new();
            mount.mount("/", Server::get_static_file)
                .mount("/js/rusty-dashed.js", dashboard);
            Iron::new(mount).http("0.0.0.0:3000").unwrap();
        }); 

        let mut webSocket = WebSocket::new(|out| {
            WsServer{client: out}
        }).unwrap();
        let broadcast = webSocket.broadcaster();

        spawn(move || {
            webSocket.listen("0.0.0.0:3001").unwrap();
        });

        let (tx, rx) = channel();

        let server = spawn(move || {
            loop {
                let message = rx.recv().unwrap();
                broadcast.send(message);
            }
        });

        (tx, server)
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

