use std::fs;
use std::path::Path;

fn main() {
    // Path to the `src/problems` directory
    let problems_dir = Path::new("src/problems");

    // Path where `mod.rs` will be generated
    let mod_file_path = problems_dir.join("mod.rs");

    let mut module_definitions = String::new();
    let mut registry_entries = String::new();

    // Iterate over files in the `problems/` directory
    for entry in fs::read_dir(problems_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check for `.rs` files with `qXX` naming convention
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Some(file_stem) = path.file_stem().and_then(|s| s.to_str()) {
                if file_stem.starts_with('q') {
                    let problem_number = file_stem.strip_prefix('q').unwrap();
                    module_definitions.push_str(&format!("pub mod {};\n", file_stem));
                    registry_entries.push_str(&format!(
                        "problems.insert(\"{}\".to_string(), {}::solution as ProblemFn);\n",
                        problem_number, file_stem
                    ));
                }
            }
        }
    }

    // Generate the `mod.rs` content
    let mod_content = format!(
        r#"
use std::collections::HashMap;

pub type ProblemFn = fn();

{module_definitions}

pub fn get_problem_function(problem_number: &str) -> Option<ProblemFn> {{
    let mut problems = HashMap::new();
    {registry_entries}
    problems.get(problem_number).copied()
}}
"#,
        module_definitions = module_definitions,
        registry_entries = registry_entries,
    );

    // Write the generated content to `mod.rs`
    fs::write(mod_file_path, mod_content).unwrap();
}
