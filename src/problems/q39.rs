// 39. Combination Sum

// Given an array of distinct integers candidates and a target integer target, return a list of all unique combinations of candidates where the chosen numbers sum to target. You may return the combinations in any order.

// The same number may be chosen from candidates an unlimited number of times. Two combinations are unique if the
// frequency
// of at least one of the chosen numbers is different.

// The test cases are generated such that the number of unique combinations that sum up to target is less than 150 combinations for the given input.

pub fn solution(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    fn backtrack(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        current_combination: &mut Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if target == 0 {
            results.push(current_combination.clone());
            return;
        }
        if target < 0 {
            return;
        }
        for i in start..candidates.len() {
            current_combination.push(candidates[i]);
            backtrack(
                candidates,
                target - candidates[i],
                i,
                current_combination,
                results,
            );
            current_combination.pop();
        }
    }

    let mut results = Vec::new();
    let mut current_combination = Vec::new();
    backtrack(
        &candidates,
        target,
        0,
        &mut current_combination,
        &mut results,
    );
    results
}

mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_solution_case() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let output = solution(candidates, target);
        assert_eq![output, vec![vec![2, 2, 3], vec![7]]];
    }

    #[test]
    fn test_solution_case2() {
        let candidates = vec![2, 3, 5];
        let target = 8;
        let output = solution(candidates, target);
        assert_eq![output, vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]];
    }

    #[test]
    fn test_solution_case3() {
        let candidates = vec![2];
        let target = 1;
        let output = solution(candidates, target);
        let correct_output: Vec<Vec<i32>> = vec![];
        assert_eq![output, correct_output];
    }
}
