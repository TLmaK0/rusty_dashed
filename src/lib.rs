#[macro_use]
extern crate lazy_static;

pub use self::dashboard::Dashboard;
pub use self::server::Server;
pub use self::ws_server::WsServer;
pub use rand::random;

mod telemetry;
mod dashboard;
mod server;
mod ws_server;
