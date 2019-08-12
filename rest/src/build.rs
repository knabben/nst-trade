extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
    println!("building");
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src/",
        input: &["protos/payload.proto"],
        includes: &["protos/"],
        customize: Customize {
            ..Default::default()
        },
    }).expect("protoc");
}