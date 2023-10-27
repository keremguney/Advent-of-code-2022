use std::fs;
use std::collections::HashSet;

pub fn run(input: &str) -> i32 {
    let file = fs::read_to_string(input).expect("failed to read file");
    let rucksack = file.lines().collect::<Vec<&str>>();
    let mut total = 0;

    for line in rucksack {
        let (left, right) = line.split_at(line.len() / 2);
        
        let left_items: HashSet<char> = HashSet::from_iter(left.chars());
        let right_items: HashSet<char> = HashSet::from_iter(right.chars());

        let common = left_items.intersection(&right_items).collect::<Vec<&char>>()[0];

        total = total + priority(*common);
    }
    total
}

fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        c as i32 - ('a' as i32) + 1
    } else {
        c as i32 - ('A' as i32) + 27
    }
}
