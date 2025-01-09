
use std::collections::HashMap;

pub type ProblemFn = fn();

pub mod q20;
pub mod q22_test;


pub fn get_problem_function(problem_number: &str) -> Option<ProblemFn> {
    let mut problems = HashMap::new();
    problems.insert("20".to_string(), q20::solution as ProblemFn);
problems.insert("22_test".to_string(), q22_test::solution as ProblemFn);

    problems.get(problem_number).copied()
}
