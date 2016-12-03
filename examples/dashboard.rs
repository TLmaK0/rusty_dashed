extern crate rusty_dashed;
use rusty_dashed::Dashboard;
use std::thread::spawn;
fn main() {

    let server = spawn(move || {
        let mut dashboard = Dashboard::new();
        dashboard.add_graph("a1", "test1", 0, 0, 4, 4);
        dashboard.add_graph("a2", "test2", 4, 0, 4, 4);
        dashboard.add_graph("a3", "test1", 8, 0, 4, 4);
        dashboard.add_graph("a4", "test2", 0, 4, 4, 4);

        let rusty_dashed = rusty_dashed::Server::new(dashboard);

        let http = rusty_dashed.start();
    });
    server.join();
}
