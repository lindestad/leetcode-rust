mod problems;

use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the problem number as an argument, e.g., `cargo run 20`");
        return;
    }

    let problem_number = &args[1];

    // Run unit tests for the specific problem
    let module_name = format!("problems::q{}", problem_number);

    println!("Running tests for problem {}", problem_number);

    let output = Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg(&module_name)
        .arg("--color")
        .arg("always") // Force color in output
        .spawn()
        .expect("Failed to execute cargo test")
        .wait_with_output()
        .expect("Failed to wait on cargo test");

    // Print the test output with colors preserved
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
}
