Rust realtime telemetry with http server, dashboard and websockets all in one.

** Run example **
`cargo run --example dashboard --features=telemetry`

then go to http://localhost:3000 and wait few seconds and see the dashboard telemetry

**  How to use **
Import the library in your Cargo.toml
```
[dependencies]
rusty_dashed = *
```

and add telemetry macro to your code:
```
#[macro_use]
extern crate rusty_dashed;

fn main(){
  let mut dashboard = Dashboard::new();
  dashboard.add_graph("mydashboard_id", "myd3jsFunction", 0, 0, 4, 4);

  let throttle = 0.01; #only 1 percent of the messages will be sended (1 to all)
  telemetry!("mydashboard_id", throttle, format!("[{myid:'id1', myvalue:1}]")

}
```

then create a folder `graphs` with two files `myd3jsFunction.js` and `myd3jsFunction.css`


*development*
run `cargo run --example dashboard --features debug_static --no-default-features` to edit public files without rebuild
