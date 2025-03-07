pub fn gcd_of_strings(str1: String, str2: String) -> String {
    let mut prefix_len = 0;
    for (a, b) in str1.chars().zip(str2.chars()) {
        if a == b {
            prefix_len += 1;
            continue;
        } else {
            break;
        }
    }

    if prefix_len == 0 {
        return String::from("");
    }

    while prefix_len > 0 {
        if (str1.len() % prefix_len != 0) || (str2.len() % prefix_len != 0) {
            prefix_len -= 1;
            continue;
        }

        let mut a = str1.split(&str1[0..prefix_len]);
        let mut b = str2.split(&str2[0..prefix_len]);

        if a.all(|s| s.is_empty()) && b.all(|s| s.is_empty()) {
            return String::from(&str1[0..prefix_len]);
        } else {
            prefix_len -= 1;
        }
    }
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let str1 = String::from("ABCABC");
        let str2 = String::from("ABC");
        let solution = String::from("ABC");

        let result = gcd_of_strings(str1, str2);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_example2() {
        let str1 = String::from("ABABAB");
        let str2 = String::from("ABAB");
        let solution = String::from("AB");

        let result = gcd_of_strings(str1, str2);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_example3() {
        let str1 = String::from("LEET");
        let str2 = String::from("CODE");
        let solution = String::from("");

        let result = gcd_of_strings(str1, str2);
        assert_eq!(result, solution);
    }
}
