use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.to_str().unwrap();

    let mc_data_path = Path::new("../dune_data_gen/minecraft-data")
        .canonicalize()
        .unwrap();
    let depends = dune_data_gen::run(out_path, &mc_data_path);

    for i in depends {
        println!("cargo:rerun-if-changed={}", i.to_str().unwrap());
    }
}
