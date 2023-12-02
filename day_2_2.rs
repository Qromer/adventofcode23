

use std::fs::read_to_string;

struct Round {
    red: usize,
    green: usize,
    blue: usize,
}


fn main() {
    let mut game_powers: Vec<usize> = Vec::new();
    let input = read_to_string("input.txt").unwrap();

    for line in input.lines() {
        let mut round = Round { red: 0, green: 0, blue: 0};
        let line = line.trim_start_matches("Game ").trim_start_matches(char::is_numeric).trim_start_matches(": ");
        let rounds = line.split("; ");
        
        for turn in rounds {
            let colors = turn.split(", ");
            for color in colors {
                let mut color = color.split_whitespace();
                let amount = color.next().unwrap().parse::<usize>().unwrap();
                
                match color.next().unwrap() {
                    "red" => if round.red < amount {round.red = amount},
                    "green" => if round.green < amount {round.green = amount},
                    "blue" => if round.blue < amount {round.blue = amount},
                    _ => (),
                }
            }
        }
        game_powers.push(round.red * round.green * round.blue);
    }
    println!("{}",game_powers.iter().sum::<usize>());
}
