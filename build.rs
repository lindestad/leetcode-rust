use std::fs;
use std::path::Path;

fn main() {
    let problems_dir = Path::new("src/problems");
    let mod_file_path = problems_dir.join("mod.rs");

    let mut module_definitions = String::new();

    for entry in fs::read_dir(problems_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                if file_stem.starts_with('q') {
                    module_definitions.push_str(&format!("pub mod {};\n", file_stem));
                }
            }
        }
    }

    let mod_content = format!(
        r#"
#![allow(dead_code)] // Suppress dead code warnings

{module_definitions}
"#,
        module_definitions = module_definitions,
    );

    fs::write(mod_file_path, mod_content).unwrap();
}
