extern crate pkg_config;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    pkg_config::probe_library("squash-0.8").unwrap();
}
