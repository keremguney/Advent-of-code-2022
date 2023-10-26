use std::fs;

pub fn most_calories(input: &str) -> u32 {
    let input = match fs::read_to_string("input.txt") {
        Ok(s) => s,
        Err(_) => panic!("corrupted file"),
    };
    let parts = input.split("\n\n")
        .map(|part| {
            part.split("\n")
                .filter(|s| !s.is_empty())
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum()
        });
    parts.max().unwrap()
}
