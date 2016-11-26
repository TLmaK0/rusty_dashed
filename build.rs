extern crate includedir_codegen;

use includedir_codegen::Compression;

fn main(){
    includedir_codegen::start("PUBLIC")
        .dir("public", Compression::Gzip)
        .build("public.rs")
        .unwrap();
}
