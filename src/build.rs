use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let protos: [&str; 7] = [
        "protos/service.proto",
        "protos/session.proto",
        "protos/network.proto",
        "protos/config.proto",
        "protos/run.proto",
        "protos/results.proto",
        "protos/uuid.proto",
    ];

    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path(out_dir.join("macro_sim_service.bin"))
        .compile_protos(&protos, &["protos"])
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    for p in &protos {
        println!("cargo:rerun-if-changed={}", p);
    }
}
