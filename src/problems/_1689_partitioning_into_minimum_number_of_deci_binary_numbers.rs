impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut res: i32 = 0;

        for c in n.chars() {
            res = std::cmp::max(res, ((c as u8) - 48) as i32);
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
    #[case("32", 3)]
    #[case("82734", 8)]
    #[case("27346209830709182346", 9)]
    fn case(#[case] n: String, #[case] expected: i32) {
        let actual = Solution::min_partitions(n);
        assert_eq!(actual, expected);
    }
}
