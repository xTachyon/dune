use std::{path::Path, time::Instant};

fn main() {
    let start = Instant::now();
    dune_data_gen::run("target", Path::new("dune_data_gen/minecraft-data"));
    println!("elapsed: {:?}", start.elapsed());
}
