//! Turns titles into URL slugs. Extras live behind Cargo features.

/// The slug of `title`: ASCII letters and digits lowercased, every other run of characters one `-`, no `-` at either end.
#[cfg(not(feature = "unicode"))]
pub fn slug(title: &str) -> String {
    ascii_slug(title)
}

/// The slug of `title`, with accents folded first.
/// CLUE: Build with `--no-default-features`, then with `--features unicode`. Does `slug("Café Menu")` give the same answer? Another crate in the same build can turn `unicode` on for you.
#[cfg(feature = "unicode")]
pub fn slug(title: &str) -> String {
    ascii_slug(&fold_accents(title))
}

/// `slug`, after folding accents: "é" becomes "e".
#[cfg(feature = "unicode")]
pub fn slug_unicode(title: &str) -> String {
    slug(title)
}

/// CLUE: `unicode` is in `default`. Which crates does a consumer that writes just `version = "0.1"` compile? Run `cargo tree`.
#[cfg(feature = "unicode")]
fn fold_accents(title: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    title
        .nfkd()
        .filter(|c| !unicode_normalization::char::is_combining_mark(*c))
        .collect()
}

fn ascii_slug(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut pending_dash = false;
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            if pending_dash && !out.is_empty() {
                out.push('-');
            }
            pending_dash = false;
            out.push(c.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }
    out
}

/// Slugs of `titles`, joined by commas.
/// CLUE: This function exists only while `json` is off. What happens to a crate that calls it when some other crate in the build turns `json` on?
#[cfg(not(feature = "json"))]
pub fn slugs_csv(titles: &[&str]) -> String {
    titles.iter().map(|t| slug(t)).collect::<Vec<_>>().join(",")
}

/// Slugs of `titles` as a JSON array of strings.
#[cfg(feature = "json")]
pub fn slugs_json(titles: &[&str]) -> String {
    let slugs: Vec<String> = titles.iter().map(|t| slug(t)).collect();
    serde_json::Value::from(slugs).to_string()
}
