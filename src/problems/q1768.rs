pub struct Solution;

struct ZipMixedLength<A, B> {
    iter1: A,
    iter2: B,
}
impl<A, B> ZipMixedLength<A, B> {
    fn new(iter1: A, iter2: B) -> Self {
        Self { iter1, iter2 }
    }
}

impl<A, B, C, D> Iterator for ZipMixedLength<A, B>
where
    A: Iterator<Item = C>,
    B: Iterator<Item = D>,
    C: Default,
    D: Default,
{
    type Item = (C, D);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.iter1.next(), self.iter2.next()) {
            (Some(a), Some(b)) => Some((a, b)),
            (Some(a), None) => Some((a, Default::default())),
            (None, Some(b)) => Some((Default::default(), b)),
            (None, None) => None,
        }
    }
}

// 1768. Merge Strings Alternately
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut result = String::new();

        let iter1 = word1.chars();
        let iter2 = word2.chars();

        for (a, b) in ZipMixedLength::new(iter1, iter2) {
            if a != '\0' {
                result.push(a);
            }
            if b != '\0' {
                result.push(b);
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_alternately() {
        let word1 = "abc".to_string();
        let word2 = "pqr".to_string();
        let output = Solution::merge_alternately(word1, word2);
        assert_eq!(output, "apbqcr".to_string());
    }

    #[test]
    fn test_merge_alternately2() {
        let word1 = "ab".to_string();
        let word2 = "pqrs".to_string();
        let output = Solution::merge_alternately(word1, word2);
        assert_eq!(output, "apbqrs".to_string());
    }

    #[test]
    fn test_merge_alternately3() {
        let word1 = "abcd".to_string();
        let word2 = "pq".to_string();
        let output = Solution::merge_alternately(word1, word2);
        assert_eq!(output, "apbqcd".to_string());
    }
}
