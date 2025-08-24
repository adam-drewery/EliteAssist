use std::env;
use std::path::PathBuf;

mod codegen;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Generate CSV-driven fdev_ids code
    codegen::fdev_ids::generate(&manifest_dir, &out_dir);

    // Generate INARA-derived location maps
    codegen::inara::generate(&out_dir);

    // Generate journal events (Rust port of generate_events.ps1)
    codegen::events::generate(&manifest_dir, &out_dir);
}
