use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.to_str().unwrap();

    let mc_data_path = Path::new("../dune_data_gen/minecraft-data")
        .canonicalize()
        .unwrap();
    dune_data_gen::run(out_path, &mc_data_path);

    println!("cargo:rerun-if-changed=../dune_data_gen/minecraft-data/data");
}
