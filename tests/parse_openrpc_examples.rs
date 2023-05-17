use std::fs::{read_dir, read_to_string};

use openrpc_type::OpenRpcDoc;

const EXAMPLE_DIR: &str = "example_files/openrpc-examples";

#[test]
fn smoke_test_openrpc() -> anyhow::Result<()> {
    for entry in read_dir(EXAMPLE_DIR)? {
        let entry = entry?;
        if entry.file_name().to_str().unwrap().ends_with(".json") {
            println!("{}", entry.file_name().to_str().unwrap());
            let path = entry.path();
            let doc: OpenRpcDoc = serde_json::from_str(&read_to_string(path)?)?;
            println!("{doc:#?}");
        }
    }
    Ok(())
}
