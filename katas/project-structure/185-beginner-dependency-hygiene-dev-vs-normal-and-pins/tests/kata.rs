// Reads this kata's own Cargo.toml and .gitignore. Don't edit this file.
use kata_project_structure_185::{format_count, parse_count};
use proptest::prelude::*;

const MANIFEST: &str = include_str!("../Cargo.toml");
const GITIGNORE: &str = include_str!("../.gitignore");
const SOURCES: [&str; 2] = [
    include_str!("../src/lib.rs"),
    include_str!("../src/main.rs"),
];
const TEST_HELPERS: [&str; 6] = [
    "proptest",
    "quickcheck",
    "criterion",
    "mockall",
    "insta",
    "pretty_assertions",
];

struct Dep {
    section: String,
    name: String,
    spec: String,
}

fn deps(manifest: &str) -> Vec<Dep> {
    let mut section = String::new();
    let mut out = Vec::new();
    for line in manifest.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(header) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            section = header.trim().to_string();
            continue;
        }
        if section != "dependencies" && section != "dev-dependencies" {
            continue;
        }
        if let Some((name, spec)) = line.split_once('=') {
            out.push(Dep {
                section: section.clone(),
                name: name.trim().to_string(),
                spec: spec.trim().to_string(),
            });
        }
    }
    out
}

fn version_of(spec: &str) -> Option<&str> {
    if let Some(rest) = spec.strip_prefix('"') {
        return rest.split('"').next();
    }
    let after = spec.split_once("version")?.1;
    let after = after
        .trim_start()
        .strip_prefix('=')?
        .trim_start()
        .strip_prefix('"')?;
    after.split('"').next()
}

fn version_ok(v: &str) -> bool {
    let core = v
        .strip_prefix('=')
        .or_else(|| v.strip_prefix('^'))
        .unwrap_or(v);
    let parts: Vec<&str> = core.split('.').collect();
    (2..=3).contains(&parts.len())
        && parts
            .iter()
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

fn spec_ok(spec: &str) -> bool {
    if spec.contains("git =") {
        return spec.contains("rev =");
    }
    version_of(spec).is_some_and(version_ok)
}

#[test]
fn the_policy_check_itself() {
    assert!(version_ok("=1.0.18"));
    assert!(version_ok("1.0"));
    assert!(version_ok("^2.8.3"));
    assert!(!version_ok("*"));
    assert!(!version_ok("1"));
    assert!(!version_ok(">=1.0"));
    assert!(!version_ok("1.*"));
    assert!(spec_ok(
        r#"{ version = "=1.12.0", default-features = false }"#
    ));
    assert!(!spec_ok(r#"{ git = "https://example.com/x.git" }"#));
    assert!(spec_ok(
        r#"{ git = "https://example.com/x.git", rev = "0a1b2c3" }"#
    ));
    assert_eq!(
        deps("[dependencies]\na = \"1.0\"\n[dev-dependencies]\nb = \"2.0\"\n").len(),
        2
    );
}

#[test]
fn test_helpers_are_dev_dependencies() {
    for d in deps(MANIFEST)
        .iter()
        .filter(|d| d.section == "dependencies")
    {
        assert!(
            !TEST_HELPERS.contains(&d.name.as_str()),
            "{} is a test helper listed in [dependencies]",
            d.name
        );
    }
    assert!(
        deps(MANIFEST)
            .iter()
            .any(|d| d.section == "dev-dependencies" && d.name == "proptest")
    );
}

#[test]
fn every_version_follows_the_policy() {
    for d in deps(MANIFEST) {
        assert!(
            spec_ok(&d.spec),
            "[{}] {} = {} breaks the README's version policy",
            d.section,
            d.name,
            d.spec
        );
    }
}

#[test]
fn every_dependency_is_used_by_the_source() {
    for d in deps(MANIFEST)
        .iter()
        .filter(|d| d.section == "dependencies")
    {
        let ident = d.name.replace('-', "_");
        assert!(
            SOURCES.iter().any(|s| s.contains(&format!("{ident}::"))),
            "{} is in [dependencies] but src/ never uses it",
            d.name
        );
    }
}

#[test]
fn the_lockfile_is_not_ignored() {
    assert!(
        !GITIGNORE
            .lines()
            .map(str::trim)
            .any(|l| l == "Cargo.lock" || l == "/Cargo.lock"),
        "this package has a binary: commit its Cargo.lock"
    );
}

#[test]
fn formats_a_count() {
    assert_eq!(format_count(0), "0");
    assert_eq!(format_count(1_234_567), "1234567");
    assert_eq!(format_count(u64::MAX), "18446744073709551615");
}

#[test]
fn parses_a_count() {
    assert_eq!(parse_count(" 42 "), Ok(42));
    assert!(parse_count("4x2").is_err());
}

proptest! {
    #[test]
    fn format_then_parse_round_trips(n in any::<u64>()) {
        prop_assert_eq!(parse_count(&format_count(n)), Ok(n));
    }
}
