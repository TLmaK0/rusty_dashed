extern crate rusty_dashed;
extern crate iron;
extern crate mount;
extern crate includedir;
extern crate phf;

use std::path::Path;
use iron::{Iron, Request, Response, IronResult};
use iron::status;
use mount::Mount;
use rusty_dashed::Dashboard;

include!(concat!(env!("OUT_DIR"), "/public.rs"));

fn get_static_file(req: &mut Request) -> IronResult<Response>{
    let file = format!("public/{}", req.url.path().join("/"));
    let file_content = match file.as_ref() {
        "public/" => PUBLIC.get("public/index.html").unwrap(),
        _ => PUBLIC.get(&file).unwrap()
    };
    
    let content = std::str::from_utf8(&file_content).unwrap();
    Ok(Response::with((status::Ok, content)))
}

fn send_rusty_dashed_js(req: &mut Request) -> IronResult<Response>{
    let mut dashboard = Dashboard::new();
    dashboard.add_graph("test/test1");
    dashboard.add_graph("test/test2");
    Ok(Response::with((status::Ok, dashboard.get_init_script())))
}

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", get_static_file)
        .mount("/js/rusty_dashed.js", send_rusty_dashed_js);
    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}
