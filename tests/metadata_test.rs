use proto_pdk_test_utils::*;

#[test]
fn registers_metadata() {
    let sandbox = create_empty_proto_sandbox();
    let plugin = sandbox.create_plugin("python-test");

    assert_eq!(
        plugin.register_tool(ToolMetadataInput::default()),
        ToolMetadataOutput {
            name: "Python".into(),
            plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
            ..ToolMetadataOutput::default()
        }
    );
}
