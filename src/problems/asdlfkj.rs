use std::{collections::BinaryHeap, f64};

pub struct Solution;

impl Solution {
    pub fn min_cost_to_hire_k_employee(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        let mut ratio: Vec<(f64, i32)> = quality
            .iter()
            .zip(wage.iter())
            .map(|(&q, &w)| (q, w as f64))
            .map(|(q, w)| (w / q as f64, q))
            .collect();

        ratio.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut res = f64::MAX;
        let mut qsum: i32 = 0;

        let mut pq: BinaryHeap<i32> = BinaryHeap::new();

        for (r, q) in ratio {
            qsum += q;
            pq.push(q);

            if pq.len() > k {
                if let Some(q_) = pq.pop() {
                    qsum -= q_
                }
                // qsum -= *pq.pop().unwrap();
            }

            if pq.len() == k {
                res = res.min(r * qsum as f64)
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_sample1() {
        assert_eq!(
            format!(
                "{:.3}",
                Solution::min_cost_to_hire_k_employee(vec![10, 20, 5], vec![70, 50, 30], 2)
            ),
            "105.000"
        );
    }

    #[test]
    fn validate_sample2() {
        assert_eq!(
            format!(
                "{:.3}",
                Solution::min_cost_to_hire_k_employee(
                    vec![3, 1, 10, 10, 1],
                    vec![4, 8, 2, 2, 7],
                    3
                )
            ),
            "30.667"
        );
    }
}
