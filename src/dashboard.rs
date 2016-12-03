pub struct Dashboard {
    graph_types: Vec<String>,
    graphs: Vec<(String, String, usize, usize, usize, usize)>
}

impl Dashboard {
    pub fn new() -> Dashboard {
        Dashboard { graph_types: vec![], graphs: vec![] }
    }

    pub fn add_graph(&mut self, id: &str, function_name: &str, x: usize, y: usize, width: usize, height: usize){
        if self.graph_types.binary_search(&function_name.to_owned()).is_err() {
            self.graph_types.push(function_name.to_owned());
        }
        self.graphs.push((id.to_owned(), function_name.to_owned(), x, y, width, height));
    }

    fn init_script<'a>(&self) -> &'a str {
        "RustyDashed.init({
          cellHeight: 80,
          verticalMargin: 10
        });
        var webSocket = new WebSocket('ws://localhost:3001');
        webSocket.onmessage = function(event) {
            eval(event.data);
        }
        "
    }

    pub fn get_init_script(&self) -> String {
        let graph_types_lines = self.graph_types.iter().map(
            |graph| format!("RustyDashed.addGraph('graphs/{}');", graph)
            ).collect::<Vec<String>>();

        let grids_lines = self.graphs.iter().map(
            |graph| format!(
                "RustyDashed.createGridItem('{}',{},{},{},{});",
                graph.0,
                graph.2,
                graph.3,
                graph.4,
                graph.5
                )
            ).collect::<Vec<String>>();

        let pass_functions = self.graphs.iter().map(
            |graph| format!(
                "function {}(data){{ {}('#{}',data); }};",
                graph.0,
                graph.1,
                graph.0
                )
            ).collect::<Vec<String>>();

        format!("{} 
                $(function(){{ {} {} {} }});",
                pass_functions.join("\n"),
                graph_types_lines.join("\n"),
                grids_lines.join("\n"),
                self.init_script()
            )
    }
}
