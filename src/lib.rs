#[macro_use] extern crate log;
extern crate env_logger;
extern crate ws;
extern crate iron;
extern crate mount;
extern crate includedir;
extern crate phf;
extern crate hyper;

pub use self::dashboard::Dashboard;
pub use self::server::Server;

mod dashboard;
mod server;
