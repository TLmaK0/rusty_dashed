use rust_embed::Embed;
use std::fs;
use std::path::Path;
use std::thread::{spawn, JoinHandle};
use crate::Dashboard;
use crate::WsServer;

#[derive(Embed)]
#[folder = "public/"]
struct Asset;

pub struct Server {
    dashboard: Dashboard,
}

impl Server {
    pub fn serve_dashboard(dashboard: Dashboard) -> JoinHandle<()> {
        let join = Server { dashboard }.start();
        WsServer::send_message("start".to_owned());
        join
    }

    fn start(&self) -> JoinHandle<()> {
        let init_script = self.dashboard.get_init_script().to_owned();
        spawn(move || {
            let server = tiny_http::Server::http("0.0.0.0:3000").unwrap();
            loop {
                let request = match server.recv() {
                    Ok(rq) => rq,
                    Err(_) => break,
                };

                let url = request.url().to_string();
                let path = url.trim_start_matches('/').split('?').next().unwrap_or("");

                // Route: /js/rusty-dashed.js -> dynamic init script
                if path == "js/rusty-dashed.js" {
                    let response = tiny_http::Response::from_string(&init_script)
                        .with_header(
                            tiny_http::Header::from_bytes(
                                &b"Content-Type"[..],
                                &b"application/javascript"[..],
                            )
                            .unwrap(),
                        );
                    let _ = request.respond(response);
                    continue;
                }

                // Route: /graphs/* -> serve from filesystem
                if path.starts_with("graphs/") {
                    let file_path = Path::new(path);
                    if file_path.exists() {
                        if let Ok(content) = fs::read_to_string(file_path) {
                            let content_type = Self::content_type_for(path);
                            let response = tiny_http::Response::from_string(content).with_header(
                                tiny_http::Header::from_bytes(
                                    &b"Content-Type"[..],
                                    content_type.as_bytes(),
                                )
                                .unwrap(),
                            );
                            let _ = request.respond(response);
                            continue;
                        }
                    }
                    let _ = request.respond(tiny_http::Response::from_string("Not Found").with_status_code(404));
                    continue;
                }

                // Route: / and everything else -> embedded static files
                let asset_path = if path.is_empty() { "index.html" } else { path };

                #[cfg(feature = "serve_static")]
                {
                    if let Some(file) = Asset::get(asset_path) {
                        let content_type = Self::content_type_for(asset_path);
                        let response =
                            tiny_http::Response::from_data(file.data.to_vec()).with_header(
                                tiny_http::Header::from_bytes(
                                    &b"Content-Type"[..],
                                    content_type.as_bytes(),
                                )
                                .unwrap(),
                            );
                        let _ = request.respond(response);
                        continue;
                    }
                }

                #[cfg(feature = "debug_static")]
                {
                    let debug_path = format!("./public/{}", asset_path);
                    if Path::new(&debug_path).exists() {
                        if let Ok(content) = fs::read_to_string(&debug_path) {
                            let content_type = Self::content_type_for(asset_path);
                            let response =
                                tiny_http::Response::from_string(content).with_header(
                                    tiny_http::Header::from_bytes(
                                        &b"Content-Type"[..],
                                        content_type.as_bytes(),
                                    )
                                    .unwrap(),
                                );
                            let _ = request.respond(response);
                            continue;
                        }
                    }
                }

                let _ = request.respond(
                    tiny_http::Response::from_string("Not Found").with_status_code(404),
                );
            }
        })
    }

    fn content_type_for(path: &str) -> &'static str {
        match Path::new(path).extension().and_then(|e| e.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") | Some("json") => "application/javascript",
            Some("ico") => "image/x-icon",
            _ => "text/plain",
        }
    }
}
