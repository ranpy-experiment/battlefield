mod problems;

fn main() {
    println!("Starting!");

    println!(
        "Result: {}",
        problems::asdlfkj::Solution::min_cost_to_hire_k_employee(
            vec![10, 20, 5],
            vec![70, 50, 30],
            2
        )
    );
    println!(
        "Result: {}",
        problems::_868_binary_gap::Solution::binary_gap(22)
    );
}
