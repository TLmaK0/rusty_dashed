extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main(){
    //TODO: run build when static files changes
    //run `cargo run --example dashboard --features debug_static --no-default-features` meanwhile
    includedir_codegen::start("PUBLIC")
        .dir("public", Compression::Gzip)
        .build("public.rs")
        .unwrap();
}
