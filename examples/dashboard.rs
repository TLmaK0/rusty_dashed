#[macro_use]
extern crate rusty_dashed;
extern crate rand;
extern crate rustc_serialize;

use rustc_serialize::json;
use rusty_dashed::Dashboard;

#[derive(RustcEncodable)]
struct Node {
    id: String,
    group: i64
}

#[derive(RustcEncodable)]
struct Link {
    source: String,
    target: String,
    value: f64
}

#[derive(RustcEncodable)]
struct Net {
    nodes: Vec<Node>,
    links: Vec<Link>,
    next: i64
}

impl Net {
    fn new() -> Net {
        Net { nodes: vec![], links: vec![], next: 0 }
    }

    fn add_node(&mut self, group: i64){
        self.nodes.push(Node { id: Net::get_node_name(self.next), group: group });
        self.next += 1;
    }

    fn add_link(&mut self, source: i64, target: i64, value: f64){
        self.links.push(Link { source: Net::get_node_name(source), target: Net::get_node_name(target), value: value });
    }

    fn get_node_name(node_id: i64) -> String{
        format!("node{}", node_id)
    }

    pub fn generate_node(&mut self){
        let group = (rand::random::<f64>() * 20.0).round() as i64;
        self.add_node(group);
        
        let source = self.next - 1;
        let mut target = source;
        while target == source {
            target = (rand::random::<f64>() * (self.next as f64 - 1.0)).round() as i64;
        }
        self.add_link(source, target, rand::random::<f64>());
    }
}

fn main() {
    let mut net = Net::new();
    net.add_node(1);
    net.add_node(2);
    net.add_node(3);
    net.add_link(1, 0, 0.1);
    net.add_link(2, 0, 0.8);
    net.add_link(1, 2, 0.4);

    let mut dashboard = Dashboard::new();
    dashboard.add_graph("a1", "test1", 0, 0, 4, 4);
    dashboard.add_graph("a2", "test2", 4, 0, 4, 4);

    rusty_dashed::Server::serve_dashboard(dashboard);

    loop {
        net.generate_node();
        telemetry!("a1", 1.0, json::encode(&net).unwrap());

        for _ in 0..10 {
            telemetry!("a2", 1.0, rand::random::<f64>().to_string());
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}
