use std::fs;

pub fn most_calories(input: &str) -> u32 {
    let input = match fs::read_to_string(input) {
        Ok(s) => s,
        Err(_) => panic!("corrupted file"),
    };
    let mut parts = input.split("\n\n")
        .map(|part| {
            part.split("\n")
                .filter(|s| !s.is_empty())
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum()
        }).collect::<Vec<u32>>();

    parts.sort();
    parts.reverse();
    parts.truncate(3);

    parts.iter().sum()
}
