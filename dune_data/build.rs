use std::{env, path::PathBuf};

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.to_str().unwrap();

    dune_data_gen::run(out_path);

    println!("cargo:rerun-if-changed=../dune_data_gen");
}
