fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto");
    let fds = protox::compile(["v1/account.proto", "v2/account.proto"], ["proto"])?;
    tonic_prost_build::configure().compile_fds(fds)?;
    Ok(())
}
