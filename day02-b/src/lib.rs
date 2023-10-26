use std::fs;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Res {
    Win,
    Draw,
    Lose,
}

pub fn run(input: &str) -> i32 {
    let bindings = fs::read_to_string(input).expect("failed to read file");
    let lines = bindings.lines();
    let mut total = 0;

    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        let opponent_choice = match split[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!("Invalid choice"),
        };
        
        let result = match split[1] {
            "X" => Res::Lose,
            "Y" => Res::Draw,
            "Z" => Res::Win,
            _ => panic!("Invalid choice"),
        };
        
        let my_choice = my_choice(&opponent_choice, result);
        total = total + calc_score(my_choice, opponent_choice);
    }
    total
}

fn calc_score(my_choice: Choice, opponent_choice: Choice) -> i32 {
    let choice_value = match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    let match_value = match (my_choice, opponent_choice) {
        (Choice::Rock, Choice::Rock) | (Choice::Paper, Choice::Paper) | (Choice::Scissors, Choice::Scissors)
            => 3,
        (Choice::Paper, Choice::Rock) | (Choice::Rock, Choice::Scissors) | (Choice::Scissors, Choice::Paper)
            => 6,
        (Choice::Scissors, Choice::Rock) | (Choice::Paper, Choice::Scissors) | (Choice::Rock, Choice::Paper)
            => 0,
    };
    choice_value + match_value
}

fn my_choice(opponent_choice: &Choice, result: Res) -> Choice {
    match (opponent_choice, result) {
        (Choice::Rock, Res::Win) | (Choice::Paper, Res::Draw) | (Choice::Scissors, Res::Lose) =>
            Choice::Paper,
        (Choice::Paper, Res::Win) | (Choice::Scissors, Res::Draw) | (Choice::Rock, Res::Lose) => 
            Choice::Scissors,
        (Choice::Scissors, Res::Win) | (Choice::Rock, Res::Draw) | (Choice::Paper, Res::Lose) =>
            Choice::Rock,
    }
}
