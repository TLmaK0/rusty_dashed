pub struct Dashboard {
    graphs: Vec<String>
}

impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard { graphs: vec![] }
    }

    pub fn add_graph(&mut self, path: &str){
        self.graphs.push(path.to_string());
    }

    pub fn get_init_script(&self) -> String {
        let script_lines = self.graphs.iter().map(|graph| format!("RustyDashed.addGraph('{}');", graph)).collect::<Vec<String>>();
        script_lines.join("\n")
    }
}
