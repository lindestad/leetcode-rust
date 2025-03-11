// 1. Two Sum
// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

pub fn fast_solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_case1() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let output = Solution::two_sum(input, target);
        assert_eq!(output, vec![0, 1]);
    }

    #[test]
    fn test_solution_case2() {
        let input = vec![3, 2, 4];
        let target = 6;
        let output = Solution::two_sum(input, target);
        assert_eq!(output, vec![1, 2]);
    }

    #[test]
    fn test_solution_case3() {
        let input = vec![3, 3];
        let target = 6;
        let output = Solution::two_sum(input, target);
        assert_eq!(output, vec![0, 1]);
    }

    #[test]
    fn test_fast_solution_case1() {
        let input = vec![2, 7, 11, 15];
        let target = 9;
        let output = fast_solution(input, target);
        assert_eq!(output, vec![0, 1]);
    }

    #[test]
    fn test_fast_solution_case2() {
        let input = vec![3, 2, 4];
        let target = 6;
        let output = fast_solution(input, target);
        assert_eq!(output, vec![1, 2]);
    }

    #[test]
    fn test_fast_solution_case3() {
        let input = vec![3, 3];
        let target = 6;
        let output = fast_solution(input, target);
        assert_eq!(output, vec![0, 1]);
    }
}
