

use std::fs::read_to_string;



const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;


struct Round {
    red: usize,
    green: usize,
    blue: usize,
}


impl Round {
    fn valid(&self) -> bool {
        if self.red > MAX_RED || self.green > MAX_GREEN || self.blue > MAX_BLUE {return false;}
        true
    }
}

fn main() {
    
    let input = read_to_string("input.txt").unwrap();

    let mut id: usize = 0;

    let mut valid_games: Vec<usize> = Vec::new();

    for line in input.lines() {
        
        let mut valid: bool = true;
        let line = line.trim_start_matches("Game ")
        .trim_start_matches(char::is_numeric)
        .trim_start_matches(": ");
        id += 1;
        let rounds = line.split("; ");
        
        for turn in rounds {
            
            let mut round = Round { red: 0, green: 0, blue: 0};
            let colors = turn.split(", ");
            

            for color in colors {
                let mut color = color.split_whitespace();
                let amount = color.next().unwrap().parse::<usize>().unwrap();
                match color.next().unwrap() {
                    "red" => round.red = amount,
                    "green" => round.green = amount,
                    "blue" => round.blue = amount,
                    _ => (),
                }
                println!{"{}",amount};

            }
            

            match round.valid() {
                true => (),
                false => valid = false,
            }
        }
        
        if let true = valid { valid_games.push(id) };
    }

    println!("{}", valid_games.iter().sum::<usize>())
}
