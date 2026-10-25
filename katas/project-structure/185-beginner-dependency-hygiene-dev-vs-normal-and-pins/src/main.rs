/// CLUE: This package builds a binary. What does `git check-ignore -v Cargo.lock` print, and which versions does a fresh clone build without a lockfile?
fn main() {
    let n = match std::env::args().nth(1) {
        Some(arg) => kata_project_structure_185::parse_count(&arg).unwrap_or(0),
        None => 0,
    };
    println!("{}", kata_project_structure_185::format_count(n));
}
