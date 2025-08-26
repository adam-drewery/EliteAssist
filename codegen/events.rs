mod r#override;
mod json;
mod rust;
mod text;
mod dedupe;

use std::fs;
use std::io::Write;
use std::path::Path;

pub fn generate(manifest_dir: &Path, out_dir: &Path) {

    let schema_root = manifest_dir.join("journal-schemas").join("schemas");
    let schemas = json::load_schemas(schema_root.as_path()).expect("schemas to load");

    // Filter out base Event schema
    let schemas: Vec<_> = schemas.into_iter()
        .filter(|schema| schema.title.as_deref() != Some("Event"))
        .collect();

    let output = rust::build(schemas).expect("rust types to build");

    let output_path = out_dir.join("event.rs");
    let output_path = output_path.to_str().expect("utf8 path");
    let mut f = fs::File::create(&output_path).expect("file created");
    f.write_all(output.to_string().as_bytes()).expect("file written");
}