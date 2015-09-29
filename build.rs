extern crate pkg_config;

fn main() {
    pkg_config::find_library("squash-0.7").unwrap();
}
