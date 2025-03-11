// 22. Generate Parentheses
// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn backtrack(result: &mut Vec<String>, current: String, open: i32, close: i32, max: i32) {
            if current.len() == (max * 2) as usize {
                result.push(current);
                return;
            }

            if open < max {
                backtrack(result, format!("{}(", current), open + 1, close, max);
            }
            if close < open {
                backtrack(result, format!("{})", current), open, close + 1, max);
            }
        }

        let mut result = Vec::new();
        backtrack(&mut result, String::new(), 0, 0, n);
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_case() {
        let input = 3;
        let output = Solution::generate_parenthesis(input);
        assert_eq![
            output,
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ]
        ];
    }

    #[test]
    fn test_solution_case2() {
        let input = 1;
        let output = Solution::generate_parenthesis(input);
        assert_eq![output, vec!["()".to_string()]];
    }
}
