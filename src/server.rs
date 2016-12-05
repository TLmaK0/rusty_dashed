use iron::headers::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use mount::Mount;
use std::path::Path;
use std::thread::{spawn, JoinHandle};
use iron::middleware::Handler;
use staticfile::Static;
use std::fs::File;
use std::io::Read;
use Dashboard;
use WsServer;

include!(concat!(env!("OUT_DIR"), "/public.rs"));

pub struct Server {
    dashboard: Dashboard,
}

lazy_static! {
}

impl Server {
    pub fn serve_dashboard(dashboard: Dashboard) -> JoinHandle<()> {
        let join = Server{ dashboard: dashboard }.start();
        WsServer::send_message("start".to_owned());
        join
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

    fn start(&self) -> JoinHandle<()> {
        let dashboard = DashboardMount{dashboard: self.dashboard.get_init_script().to_owned()};
        let server = spawn(move || {
            let mut mount = Mount::new();
            mount
                .mount("/", Server::get_static_file)
                .mount("/graphs/", Static::new(Path::new("graphs")))
                .mount("/js/rusty-dashed.js", dashboard);
            Iron::new(mount).http("0.0.0.0:3000").unwrap();
        }); 
        server
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

