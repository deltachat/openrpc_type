use openrpc_type::OpenRpcDoc;

#[test]
fn smoke_test_yerpc() {
    let doc: OpenRpcDoc =
        serde_json::from_str(include_str!("../example_files/yerpc_axum_openrpc.json")).unwrap();
    eprintln!("{doc:#?}");
}
