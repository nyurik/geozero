#[cfg(feature = "with-mvt")]
use std::{
    env,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};

#[cfg(feature = "with-mvt")]
fn compile_protos() -> Result<(), Box<dyn std::error::Error>> {
    // override the build location, in order to check in the changes to proto files
    env::set_var("OUT_DIR", "src/mvt");

    // The current working directory can vary depending on how the project is being
    // built or released so we build an absolute path to the proto file
    let path = Path::new("src/mvt/vector_tile.proto");
    if path.exists() && cfg!(feature = "with-mvt") && env::var("DOCS_RS").is_err() {
        // avoid rerunning build if the file has not changed
        println!("cargo:rerun-if-changed=src/mvt/vector_tile.proto");

        prost_build::compile_protos(&["src/mvt/vector_tile.proto"], &["src/mvt/"])?;
        // read file contents to string
        let mut file = OpenOptions::new()
            .read(true)
            .open("src/mvt/vector_tile.rs")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        // append warning that file was auto-generate
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("src/mvt/vector_tile.rs")?;
        file.write_all("// This file was automatically generated through the build.rs script, and should not be edited.\n\n".as_bytes())?;
        file.write_all(buffer.as_bytes())?;
    }

    // As the proto file is checked in, the build should not fail if the file is not found
    Ok(())
}

#[cfg(feature = "with-mvt")]
fn compile_protos2() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = ["src/mvt/vector_tile.proto"];

    for path in &proto_files {
        println!("cargo:rerun-if-changed={path}");
    }

    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = format!("{out_dir}/tile2");

    protobuf_codegen::Codegen::new()
        .pure()
        .out_dir(&out_dir)
        .inputs(proto_files)
        .include("src/mvt")
        .run()?;

    Ok(())
}

#[cfg(feature = "with-mvt")]
fn compile_protos3() -> Result<(), Box<dyn std::error::Error>> {
    use pb_rs::{types::FileDescriptor, ConfigBuilder};
    use std::path::PathBuf;

    let proto_files = [PathBuf::from("src/mvt/vector_tile.proto")];

    let out_dir = std::env::var("OUT_DIR")?;
    let out_dir = PathBuf::from(format!("{out_dir}/tile3"));

    let in_dir = PathBuf::from(::std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("tile3");

    // Delete all old generated files before re-generating new ones
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).unwrap();
    }
    std::fs::DirBuilder::new().create(&out_dir).unwrap();
    let config_builder = ConfigBuilder::new(&proto_files, None, Some(&out_dir), &[in_dir]).unwrap();
    FileDescriptor::run(&config_builder.build()).unwrap();

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "with-mvt")]
    compile_protos()?;

    #[cfg(feature = "with-mvt")]
    compile_protos2()?;

    #[cfg(feature = "with-mvt")]
    compile_protos3()?;

    Ok(())
}
