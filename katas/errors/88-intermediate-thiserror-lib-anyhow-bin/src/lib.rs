//! Loading service settings from a key = value file.

use anyhow::{Context, bail};
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub port: u16,
    pub workers: u32,
}

/// CLUE: A caller gets `anyhow::Error` back. How would it tell a missing key from a bad number without comparing strings?
pub fn parse_settings(text: &str) -> anyhow::Result<Settings> {
    let mut port = None;
    let mut workers = None;
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n = index + 1;
        let Some((key, value)) = line.split_once('=') else {
            bail!("line {n}: expected `key = value` with a known key");
        };
        let (key, value) = (key.trim(), value.trim());
        match key {
            // CLUE: Where does the `ParseIntError` go after this line, and can a caller get at it as a value?
            "port" => port = Some(value.parse::<u16>().context("bad port")?),
            "workers" => workers = Some(value.parse::<u32>().context("bad workers")?),
            _ => bail!("line {n}: expected `key = value` with a known key"),
        }
    }
    Ok(Settings {
        port: port.context("missing port")?,
        workers: workers.context("missing workers")?,
    })
}

pub fn load_settings<R: Read>(mut input: R) -> anyhow::Result<Settings> {
    let mut text = String::new();
    input.read_to_string(&mut text)?;
    parse_settings(&text)
}
