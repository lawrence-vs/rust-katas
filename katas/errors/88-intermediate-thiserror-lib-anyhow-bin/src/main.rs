use kata_errors_88::load_settings;
use std::fs::File;
use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: kata-errors-88 <settings.conf>");
        return ExitCode::from(2);
    };
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    match load_settings(file) {
        Ok(settings) => {
            println!(
                "listening on port {} with {} workers",
                settings.port, settings.workers
            );
            ExitCode::SUCCESS
        }
        // CLUE: `{e}` prints one message. What does `{e:?}` print for an `anyhow::Error`, and which file was being read?
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
