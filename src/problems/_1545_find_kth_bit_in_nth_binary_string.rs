#[allow(unused)]
impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        /*
         * the idea is that we can kind of binary search through 0..n to find the postition of k
         * the left most bit represents whether we need to go left or right
         * can recursively travel through the mirror states
         * if the left most bit is 0 - left half, 1 - right half
         * if the second to left most bit is 0 - left half of the half in the previous step
         * from there, we can kind of iterate/recurse through to the base state
         */

        // get right most set bit
        let bit_pos: i32 = k & -k;

        // check if it is inverted
        // one left to the kth bit 1 - inverted (right part)
        let is_inverted: bool = ((k / bit_pos) >> 1 & 1) == 1;

        // check if value before inversion of kth bit was 0 or 1
        let bit_val: bool = (k & 1) == 0;

        // invert if required
        if is_inverted ^ bit_val { '1' } else { '0' }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, 1, '0')]
    #[case(4, 11, '1')]
    fn case(#[case] n: i32, #[case] k: i32, #[case] expected: char) {
        let actual = Solution::find_kth_bit(n, k);
        assert_eq!(actual, expected);
    }
}
