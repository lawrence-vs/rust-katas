#[test]
fn no_native_tls_in_the_manifest() {
    let manifest = include_str!("../Cargo.toml");
    assert!(
        !manifest.contains("native-tls"),
        "native-tls is banned, use rustls"
    );
}

#[test]
fn ping_works() {
    assert_eq!(kata_project_structure_228::ping(), "pong");
}
