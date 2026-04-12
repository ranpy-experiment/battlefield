use itertools::Itertools;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut chars: Vec<char> = s.chars().collect();
        chars.reverse();
        chars.push(' ');

        let mut left = 0;
        let mut right = 0;
        let mut i = 0;

        let n = chars.len() - 1;

        while i < n {
            while i < n && chars[i] == ' ' {
                i += 1;
            }

            if i == n {
                break;
            }

            while i < n && chars[i] != ' ' {
                chars[right] = chars[i];
                right += 1;
                i += 1;
            }
            chars[left..right].reverse();
            chars[right] = ' ';
            right += 1;
            left = right;
            i += 1;
        }

        chars.truncate(right - 1);
        chars.iter().collect()
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
