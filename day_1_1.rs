use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut numbers: Vec<u32> = Vec::new();


    for line in input.lines() {
        let mut result = String::new();
        
        let numberstring = line.chars()
            .filter(|c| c.to_digit(10).is_some())
            .collect::<String>();
        
        let mut numberchars = numberstring.chars();
        
        if let Some(x) = numberchars.next() { result.push(x) };
        match numberchars.last() { 
            Some(x) => {result.push(x) },
            None => { result = result.clone() + &result }
        };
        
        numbers.push(result.parse::<u32>().unwrap());
    }

    println!("sum: {}", numbers.iter().sum::<u32>());
}
