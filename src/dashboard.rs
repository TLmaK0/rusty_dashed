pub struct Dashboard {
    graph_types: Vec<String>,
    graphs: Vec<(String, usize, usize, usize, usize)>
}

impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard { graph_types: vec![], graphs: vec![] }
    }

    pub fn add_graph(&mut self, path: &str, x: usize, y: usize, width: usize, height: usize){
        if self.graph_types.binary_search(&path.to_owned()).is_err() {
            self.graph_types.push(path.to_owned());
        }
        self.graphs.push((path.to_owned(), x, y, width, height));
    }

    fn init_script<'a>(&self) -> &'a str {
        "RustyDashed.init({
          cellHeight: 80,
          verticalMargin: 10
        });\n"
    }

    pub fn get_init_script(&self) -> String {
        let graph_types_lines = self.graph_types.iter().map(|graph| format!("RustyDashed.addGraph('{}');", graph)).collect::<Vec<String>>();

        let grids_lines = self.graphs.iter().map(|graph| format!("RustyDashed.createGridItem({},{},{},{});", graph.1, graph.2, graph.3, graph.4)).collect::<Vec<String>>();

        format!("$(function(){{ {} {} {} }});",
                graph_types_lines.join("\n"),
                grids_lines.join("\n"),
                self.init_script()
            )
    }
}
