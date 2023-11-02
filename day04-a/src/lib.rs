use std::fs;

pub fn run(input: &str) -> i32 {
    let input = fs::read_to_string(input).expect("failed to read file");
    let mut total = 0;
    for line in input.lines() {
        let (first, second) = line.split_once(",").unwrap();
        let (first_num1, first_num2) = first.split_once("-").unwrap();
        let (second_num1, second_num2) = second.split_once("-").unwrap();
        if first_num1.parse::<u32>().unwrap() <= second_num1.parse::<u32>().unwrap() && 
            first_num2.parse::<u32>().unwrap() >= second_num2.parse::<u32>().unwrap() || 
            first_num1.parse::<u32>().unwrap() >= second_num1.parse::<u32>().unwrap() && 
            first_num2.parse::<u32>().unwrap() <= second_num2.parse::<u32>().unwrap() {
                total += 1;
            }
    }
    total
}
