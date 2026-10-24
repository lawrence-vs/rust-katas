fn main() {
    let args = kata_cli_91::from_env();
    println!(
        "input={} limit={} format={:?} verbose={}",
        args.input.display(),
        args.limit,
        args.format,
        args.verbose
    );
}
