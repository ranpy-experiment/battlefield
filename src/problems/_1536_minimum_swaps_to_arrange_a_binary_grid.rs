impl Solution {
    pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();

        let mut trails: Vec<i32> = vec![0; n];
        for i in 0..n {
            let mut trail: i32 = 0;
            for j in (0..n).rev() {
                if grid[i][j] != 0 {
                    break;
                }
                trail += 1;
            }
            trails[i] = trail;
        }

        let mut swaps: i32 = 0;
        for i in 0..n {
            let mut j: usize = 0;
            let req: i32 = (n - i - 1) as i32;
            while j < trails.len() && trails[j] < req {
                j += 1;
            }
            if j == trails.len() {
                return -1 as i32;
            }
            swaps += j as i32;
            trails.remove(j as usize);
        }

        swaps
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,0,1],vec![1,1,0],vec![1,0,0]], 3)]
    #[case(vec![vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0],vec![0,1,1,0]], -1)]
    #[case(vec![vec![1,0,0],vec![1,1,0],vec![1,1,1]], 0)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_swaps(grid.clone());
        assert_eq!(actual, expected);
    }
}
