// 20. Valid Parentheses
// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

//     1. Open brackets must be closed by the same type of brackets.
//     2. Open brackets must be closed in the correct order.
//     3. Every close bracket has a corresponding open bracket of the same type.

pub fn solution(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' if Some(c) != stack.pop() => return false,
            _ => (),
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_case() {
        let input = "()".to_string();
        let output = solution(input);
        assert!(output);
    }

    #[test]
    fn test_solution_case2() {
        let input = "()[]{}".to_string();
        let output = solution(input);
        assert!(output);
    }

    #[test]
    fn test_solution_case3() {
        let input = "(]".to_string();
        let output = solution(input);
        assert!(!output);
    }

    #[test]
    fn test_solution_case4() {
        let input = "([])".to_string();
        let output = solution(input);
        assert!(output);
    }
}
