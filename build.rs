use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "./proto/todos.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("todos_descriptor.bin"))
        .out_dir("./src")
        .compile(&[file_path], &["proto"])?;
    Ok(())
}
