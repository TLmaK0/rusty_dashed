#[macro_use]
extern crate rusty_dashed;
extern crate rand;

use rusty_dashed::Dashboard;

fn main() {
    let data1 = r#"{
      "nodes": [
        {"id": "Myriel", "group": 1},
        {"id": "Napoleon", "group": 1},
        {"id": "Mlle.Baptistine", "group": 1}
      ],
      "links": [
        {"source": "Napoleon", "target": "Myriel", "value": 1},
        {"source": "Mlle.Baptistine", "target": "Myriel", "value": 8},
        {"source": "Napoleon", "target": "Mlle.Baptistine", "value": 4}
      ]
    }"#;

    let mut dashboard = Dashboard::new();
    dashboard.add_graph("a1", "test1", 0, 0, 4, 4);
    dashboard.add_graph("a2", "test2", 4, 0, 4, 4);

    let server = rusty_dashed::Server::serve_dashboard(dashboard);

    std::thread::sleep(std::time::Duration::from_millis(10000));
    telemetry!("a1", 1.0, data1);

    loop {
        telemetry!("a2", 1.0, rand::random::<f64>().to_string());
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
