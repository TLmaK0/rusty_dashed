#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate ws;
extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate includedir;
extern crate phf;
extern crate hyper;

pub use self::dashboard::Dashboard;
pub use self::server::Server;
pub use self::ws_server::WsServer;

mod telemetry;
mod dashboard;
mod server;
mod ws_server;
