pub fn solution(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
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
        let output = solution(input, target);
        assert_eq!(output, vec![0, 1]);
    }

    #[test]
    fn test_solution_case2() {
        let input = vec![3, 2, 4];
        let target = 6;
        let output = solution(input, target);
        assert_eq!(output, vec![1, 2]);
    }

    #[test]
    fn test_solution_case3() {
        let input = vec![3, 3];
        let target = 6;
        let output = solution(input, target);
        assert_eq!(output, vec![0, 1]);
    }
}
