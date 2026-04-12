use itertools::Itertools;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let str_val = s.trim();

        return str_val.split_whitespace().rev().join(" ");
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("the sky is blue", "blue is sky the")]
    #[case("  hello world  ", "world hello")]
    #[case("a good   example", "example good a")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::reverse_words(s);
        assert_eq!(actual, expected);
    }
}
