# 185 beginner: dependency hygiene, dev vs normal and pins

**Concept:** Test-only crates go in `[dev-dependencies]`; every version is caret-bounded or pinned, never `*`; every dependency is one `src/` uses; a package with a binary commits its `Cargo.lock`.
**Source:** The Cargo Book, "Specifying Dependencies" (idea only); the Cargo FAQ, "Why have `Cargo.lock` in version control?"
**Fails with:** `test_helpers_are_dev_dependencies`
**Vault:** your note on Cargo dependencies.

A normal dependency is built, linked and audited by every user of the package. `*` lets `cargo update` pull in a new major version without anyone choosing it. An unused crate adds build time and advisory risk for nothing. The tests read this kata's own `Cargo.toml` and `.gitignore` as text, so each rule is checked against the files you change.

## Task
The fix is in `Cargo.toml` and `.gitignore`, plus committing the `Cargo.lock` that cargo writes. Don't change `src/` or `tests/`. Use `cargo add --dev`, `cargo remove`, and `cargo add itoa@=<version>`.

## Dependency policy
1. A version is `MAJOR.MINOR` or `MAJOR.MINOR.PATCH`, optionally prefixed `^` or `=`. No `*`, no `>=`, no bare major.
2. A git dependency names a `rev`.
3. A crate used only by tests, benches or examples goes in `[dev-dependencies]`.
4. Every `[dependencies]` entry is used as `crate_name::` in `src/`.
5. This package has a binary, so `Cargo.lock` is committed.
6. One dependency per line.

## Before you fix it
1. Run `cargo test`. Which 4 tests fail, and which one does the README quote?
2. Run `cargo tree -e normal` and `cargo tree -e dev`. Which crates appear only in the second list?
3. Run `git check-ignore -v Cargo.lock`. Which line of which file ignores it?

## Answer in the PR
1. Why was each of the four a bug, and who pays for it (downstream builds, audit, reproducibility)?
2. Compare `=1.0.18` with `"1.0"` for this binary, and say what each costs.
3. What does `Cargo.lock` not protect, according to the Cargo FAQ?

## Think further
1. What does `cargo update -p itoa --precise <v>` change, and in which file?
2. What does `cargo audit` read, and why does it need the lockfile?
3. `rust-version` in `[package]` vs the repo's `rust-toolchain.toml`: what does each enforce, and for whom?
4. Two crates in one workspace both use itoa: what does `[workspace.dependencies]` plus `itoa.workspace = true` buy you?
5. When is `cargo vendor` worth it?
6. Reading: The Rust Book, "Managing Growing Projects with Packages, Crates, and Modules" and "More About Cargo and Crates.io"; the Cargo Book, "Specifying Dependencies", "Features" and "Cargo.toml vs Cargo.lock"; the Rust API Guidelines. Check each chapter title against the live site.

## In the wild
- crates.io rejects a wildcard dependency requirement when you publish.
- `cargo-machete` and `cargo-udeps` report unused dependencies.
- The Cargo FAQ section "Why have `Cargo.lock` in version control?" explains why binaries usually commit the lockfile.

In a review, check:
- a test-only crate under `[dependencies]`
- `*` or `>=` as a version
- a git dependency without `rev`
- a binary's `Cargo.lock` in `.gitignore`

## Related concepts

### `[build-dependencies]`
Crates that run in `build.rs` at compile time, not in the final program. Use them for code generation or finding a C library.
```toml
[build-dependencies]
cc = "1.2"
```

### Optional dependencies plus features
A dependency that is compiled only when a named feature turns it on. Use it for heavy crates that some users need.
```toml
[dependencies]
serde = { version = "1.0", optional = true }
[features]
json = ["dep:serde"]
```

### `[target.'cfg(...)'.dependencies]`
A dependency that is included only for one platform. Use it for OS-specific crates.
```toml
[target.'cfg(unix)'.dependencies]
libc = "0.2"
```

### `[workspace.dependencies]`
One place in the workspace root that sets a version, which member crates inherit. Use it when several crates share a dependency.
```toml
[workspace.dependencies]
serde = "1.0"
```
