use std::path::Path;

fn main() {
    dune_data_gen::run("target", Path::new("dune_data_gen/minecraft-data"));
}
