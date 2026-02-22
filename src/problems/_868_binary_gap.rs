//! Solution for https://leetcode.com/problems/binary-gap
//! 868. Binary Gap

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let binary_string = format!("{:b}", n);

        let mut res: i32 = 0;

        let mut start: i32 = -1;
        for (i, digit) in binary_string.char_indices() {
            if digit == '0' as char {
                continue;
            }

            if start == -1 {
                start = i as i32;
            } else {
                let cur: i32 = i as i32 - start;
                res = std::cmp::max(res, cur);
                start = i as i32;
            }
        }

        res
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(22, 2)]
    #[case(8, 0)]
    #[case(5, 2)]
    fn case(#[case] n: i32, #[case] expected: i32) {
        let actual = Solution::binary_gap(n);
        assert_eq!(actual, expected);
    }
}
