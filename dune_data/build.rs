use std::{
    env,
    path::{Path, PathBuf},
};

fn c(p: PathBuf) -> String {
    p.canonicalize().unwrap().to_str().unwrap().to_string()
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = out_path.to_str().unwrap();

    let mc_data_path = Path::new("../dune_data_gen/minecraft-data")
        .canonicalize()
        .unwrap();
    dune_data_gen::run(out_path, &mc_data_path);

    let p = Path::new("../dune_data_gen/minecraft-data/data");
    println!("cargo:rerun-if-changed={}", c(p.join("dataPaths.json")));
    for i in dune_data_gen::VERSIONS {
        println!("cargo:rerun-if-changed={}", c(p.join("pc").join(i)));
    }
}
