use std::env::{self, Args};

fn main() {
    let args: Args = env::args();
    imp_rust::run(args).unwrap();
}
