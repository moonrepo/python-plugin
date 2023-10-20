use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_resolve_versions_tests!("python-test", {
    "2.3" => "2.3.7",
    "3.10.1" => "3.10.1",
    "3.10" => "3.10.13",
    "3" => "3.12.0",
});

#[test]
fn loads_versions_from_git() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("python-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("python-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}
