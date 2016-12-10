#[macro_use] extern crate lazy_static;
extern crate ws;
extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate includedir;
extern crate phf;
extern crate hyper;
extern crate rand;

pub use self::dashboard::Dashboard;
pub use self::server::Server;
pub use self::ws_server::WsServer;
pub use self::rand::random;

mod telemetry;
mod dashboard;
mod server;
mod ws_server;
