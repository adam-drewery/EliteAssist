mod fixes;
mod json;
mod rust;

use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate(manifest_dir: &Path, out_dir: &Path) {
    let schema_root = manifest_dir.join("journal-schemas").join("schemas");
    let output_path = out_dir.join("event.rs");
    let output_path = output_path.to_str().expect("utf8 path not found");
    let schemas = json::load_schemas(schema_root.as_path()).expect("schemas failed to load");
    let output = rust::build(schemas).expect("rust types failed to build");
    let mut f = fs::File::create(&output_path).expect("file creation failed");
    f.write_all(output.to_string().as_bytes()).expect("file failed to write");
}