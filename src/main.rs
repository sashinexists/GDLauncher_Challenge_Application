fn main() {
    let input = include_str!("challenge_input.txt");
    let processed_input = process_input(input);
    let first_unsafe_value = find_first_unsafe_number(processed_input);
    match first_unsafe_value {
        Some(val) => println!("The first unsafe number here is {val}."),
        None => println!("All the numbers here are safe."),
    }
}

fn process_input(input: &str) -> Vec<u128> {
    input
        .lines()
        .map(|line| {
            line.parse::<u128>()
                .expect("Failed to parse line \"{line}\" as u64")
        })
        .collect()
}

fn do_any_sums_in_x_equal_y(x: Vec<u128>, y: u128) -> bool {
    for a in 0..x.len() {
        for b in (0..x.len()).rev() {
            if a != b && (x[a] + x[b]) == y {
                return true;
            }
        }
    }
    false
}

fn find_first_unsafe_number(numbers: Vec<u128>) -> Option<u128> {
    const NUMBER_OF_LINES_TO_CHECK: usize = 100;
    for i in NUMBER_OF_LINES_TO_CHECK..numbers.len() {
        if !do_any_sums_in_x_equal_y(
            numbers[i - NUMBER_OF_LINES_TO_CHECK..i - 1].to_vec(),
            numbers[i],
        ) {
            return Some(numbers[i]);
        }
    }
    None
}
