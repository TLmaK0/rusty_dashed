extern crate rusty_dashed;
extern crate iron;
extern crate mount;
extern crate includedir;
extern crate phf;
extern crate hyper;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use mount::Mount;
use rusty_dashed::Dashboard;
use iron::headers::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use std::fs::File;
use std::io::prelude::*;

include!(concat!(env!("OUT_DIR"), "/public.rs"));

fn get_static_file(req: &mut Request) -> IronResult<Response> {
    let request_path = format!("public/{}", req.url.path().join("/"));
    let file_path = match request_path.as_ref() {
        "public/" => "public/index.html",
        path => path
    };

    let content = get_file_content(&file_path); 

    let content_type = match std::path::Path::new(&file_path).extension().unwrap().to_str().unwrap() {
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

fn send_rusty_dashed_js(_req: &mut Request) -> IronResult<Response>{
    let mut dashboard = Dashboard::new();
    dashboard.add_graph("graphs/test1", 0, 0, 4, 4);
    dashboard.add_graph("graphs/test2", 4, 0, 4, 4);
    dashboard.add_graph("graphs/test1", 8, 0, 4, 4);
    dashboard.add_graph("graphs/test2", 0, 4, 4, 4);
    Ok(Response::with((status::Ok, dashboard.get_init_script())))
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", get_static_file)
        .mount("/js/rusty-dashed.js", send_rusty_dashed_js);
    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}
