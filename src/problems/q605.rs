pub struct Solution;

impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let size = flowerbed.len();
        if n == 0 {
            return true;
        }
        if size == 1 {
            return n == 1 && flowerbed[0] == 0;
        }
        let mut i: usize = 3;
        let mut counter = 0;
        if flowerbed[0] == 0 && flowerbed[1] == 0 {
            counter += 1;
        }

        while i < size {
            if flowerbed[i - 2] == 0 && flowerbed[i - 1] == 0 && flowerbed[i] == 0 {
                counter += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        if i == size && flowerbed[i - 2] == 0 && flowerbed[i - 1] == 0 {
            counter += 1;
        }

        counter >= n
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let solution = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let solution = false;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_3() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let n = 2;
        let solution = false;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_4() {
        let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
        let n = 2;
        let solution = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_5() {
        let flowerbed = vec![0];
        let n = 1;
        let solution = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_6() {
        let flowerbed = vec![0, 0, 0, 0];
        let n = 3;
        let solution = false;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, solution);
    }

    #[test]
    fn test_7() {
        let flowerbed = vec![1, 0, 0];
        let n = 1;
        let solution = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, solution);
    }
}
