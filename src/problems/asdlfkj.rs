use ordered_float::OrderedFloat;
use std::{collections::BinaryHeap, f64};

pub struct Solution;

impl Solution {
    pub fn min_cost_to_hire_k_employee(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let mut ratio: Vec<Vec<f64>> = Vec::new();

        for i in 0..quality.len() {
            ratio.push(vec![
                (wage[i] as f64) / (quality[i] as f64),
                quality[i] as f64,
            ]);
        }

        ratio.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        let mut res = f64::MAX;
        let mut qsum = 0.0;

        let mut pq: BinaryHeap<OrderedFloat<f64>> = BinaryHeap::new();

        for val in ratio {
            qsum += val[1];
            pq.push(OrderedFloat(val[1]));

            if pq.len() as i32 > k {
                qsum -= *pq.pop().unwrap();
            }
            if pq.len() as i32 == k {
                res = res.min((qsum as f64) * val[0]);
            }
        }

        res as f64
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
