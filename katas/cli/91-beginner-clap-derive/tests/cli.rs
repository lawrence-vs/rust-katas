use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kata-cli-91"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn the_binary_prints_the_parsed_arguments() {
    let output = run(&["trades.csv"]);
    assert!(output.status.success(), "status: {:?}", output.status);
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "input=trades.csv limit=10 format=Text verbose=false\n"
    );
}

#[test]
fn help_goes_to_stdout() {
    let output = run(&["--help"]);
    assert!(output.status.success(), "status: {:?}", output.status);
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Usage: kata-cli-91"), "stdout: {stdout}");
}

#[test]
fn no_arguments_exits_with_code_2() {
    let output = run(&[]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("required"), "stderr: {stderr}");
}

#[test]
fn a_bad_limit_exits_with_code_2() {
    let output = run(&["trades.csv", "--limit", "0"]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("--limit"), "stderr: {stderr}");
}
