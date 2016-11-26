extern crate rusty_dashed;
extern crate staticfile;
extern crate iron;
extern crate mount;

use std::path::Path;
use iron::Iron;
use staticfile::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("public/")));
    Iron::new(mount).http("0.0.0.0:3000").unwrap();
}
