pub struct Solution;

impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let size = candies.len();

        let max_candies = *candies.iter().max().unwrap();
        let mut bools: Vec<bool> = Vec::with_capacity(size);

        for candy in candies {
            bools.push((candy + extra_candies) >= max_candies);
        }
        bools
    }
}
#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let candies = vec![2, 3, 5, 1, 3];
        let extra_candies = 3;
        let solution = vec![true, true, true, false, true];
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, solution);
    }

    #[test]
    fn example_2() {
        let candies = vec![4, 2, 1, 1, 2];
        let extra_candies = 1;
        let solution = vec![true, false, false, false, false];
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, solution);
    }

    #[test]
    fn example_3() {
        let candies = vec![12, 1, 12];
        let extra_candies = 10;
        let solution = vec![true, false, true];
        let result = Solution::kids_with_candies(candies, extra_candies);
        assert_eq!(result, solution);
    }
}
