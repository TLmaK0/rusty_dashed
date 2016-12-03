extern crate rusty_dashed;
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
    dashboard.add_graph("a2", "test1", 4, 0, 4, 4);
    dashboard.add_graph("a3", "test1", 8, 0, 4, 4);
    dashboard.add_graph("a4", "test1", 0, 4, 4, 4);

    let (tx, server) = rusty_dashed::Server::new(dashboard).start();

    std::thread::sleep(std::time::Duration::from_millis(10000));
    tx.send(format!("a1({})", data1).to_owned()).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(format!("a2({})", data1).to_owned()).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(format!("a3({})", data1).to_owned()).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
    tx.send(format!("a4({})", data1).to_owned()).unwrap();

    server.join().unwrap();
}
