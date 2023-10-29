use std::fs;
use std::collections::HashSet;

pub fn run(input: &str) -> i32 {
    let file = fs::read_to_string(input).expect("failed to read file");
    let rucksack = file.lines().collect::<Vec<&str>>();
    let mut total = 0;

    for i in (0..rucksack.len()).step_by(3) {
        if i + 2 < rucksack.len() {
            let first: HashSet<char> = HashSet::from_iter(rucksack[i].chars());
            let second: HashSet<char> = HashSet::from_iter(rucksack[i + 1].chars());
            let third: HashSet<char> = HashSet::from_iter(rucksack[i + 2].chars());

            let common: HashSet<char> = first.intersection(&second).cloned().collect();
            let common: HashSet<char> = common.intersection(&third).cloned().collect();

            for c in common {
                total += priority(c);
            }
        }
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
