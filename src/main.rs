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
    println!(
        "Result: {}",
        problems::_1689_partitioning_into_minimum_number_of_deci_binary_numbers::Solution::min_partitions(String::from("32"))
    );
    println!(
        "Result: {}",
        problems::_1536_minimum_swaps_to_arrange_a_binary_grid::Solution::min_swaps(vec!(
            vec![0, 0, 1],
            vec![1, 1, 0],
            vec![1, 0, 0]
        ))
    );

    println!(
        "Result: {}",
        problems::_151_reverse_words_in_a_string::Solution::reverse_words(String::from(
            "the sky is blue"
        ))
    );
}
