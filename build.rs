fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/proto")
        .include_file("proto.rs")
        .compile(&["proto/main.proto"], &["proto"])?;

    Ok(())
}
