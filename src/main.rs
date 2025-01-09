mod problems;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide the problem number as an argument, e.g., `cargo run 20`");
        return;
    }

    let problem_number = &args[1];
    match problems::get_problem_function(problem_number) {
        Some(solution) => solution(),
        None => eprintln!("Problem {} not found!", problem_number),
    }
}
