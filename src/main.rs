mod problems;

use problems::asdlfkj::Solution;

fn main() {
    println!("Starting!");

    println!(
        "Result: {}",
        Solution::min_cost_to_hire_k_employee(vec![10, 20, 5], vec![70, 50, 30], 2)
    )
}
