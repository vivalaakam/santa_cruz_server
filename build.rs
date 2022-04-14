use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .file_descriptor_set_path(
            PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
                .join("file_descriptor_set.bin"),
        )
        .out_dir("src/proto")
        .include_file("proto.rs")
        .compile(&["proto/main.proto"], &["proto"])?;

    Ok(())
}
