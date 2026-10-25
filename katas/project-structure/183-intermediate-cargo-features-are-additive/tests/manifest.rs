const MANIFEST: &str = include_str!("../Cargo.toml");

fn features_table() -> &'static str {
    let start = MANIFEST
        .find("[features]")
        .expect("Cargo.toml has a [features] table");
    let rest = &MANIFEST[start + "[features]".len()..];
    let end = rest.find("\n[").unwrap_or(rest.len());
    &rest[..end]
}

#[test]
fn default_features_are_empty() {
    for line in features_table().lines() {
        if line.trim_start().starts_with("default") {
            assert_eq!(
                line.trim(),
                "default = []",
                "the default feature set must be empty, so a consumer gets no extra crates"
            );
        }
    }
}

#[test]
fn optional_dependencies_use_dep_prefix() {
    for dep in ["serde_json", "unicode-normalization"] {
        assert!(
            features_table().contains(&format!("\"dep:{dep}\"")),
            "enable {dep} as \"dep:{dep}\""
        );
    }
}

#[test]
fn no_implicit_dependency_features() {
    for dep in ["serde_json", "unicode-normalization"] {
        assert!(
            !features_table().contains(&format!("\"{dep}\"")),
            "\"{dep}\" without dep: also makes a public feature named {dep}"
        );
    }
}

#[test]
fn the_features_and_optional_deps_are_kept() {
    let table = features_table();
    assert!(
        table
            .lines()
            .any(|l| l.trim_start().starts_with("unicode =")),
        "keep the unicode feature"
    );
    assert!(
        table.lines().any(|l| l.trim_start().starts_with("json =")),
        "keep the json feature"
    );
    assert_eq!(
        MANIFEST.matches("optional = true").count(),
        2,
        "keep both optional dependencies"
    );
}
