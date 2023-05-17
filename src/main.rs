use std::{env::args, fs::read_to_string};

use openrpc_type::OpenRpcDoc;

fn main() {
    let mut args = args();
    args.next(); // first argument is always exe itself
    let file = args.next().expect("no file set");

    let content = read_to_string(file).expect("failed to read file");
    let doc: OpenRpcDoc = serde_json::from_str(&content).unwrap();
    println!("{doc:#?}");
}
