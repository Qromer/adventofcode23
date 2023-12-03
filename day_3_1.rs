


use std::fs::read_to_string;
use std::option::Option;
fn main() {
    
    let input = read_to_string("input.txt").unwrap();
    
    let mut output: Vec<u32> = Vec::new();
    let mut numbers: Vec<Vec<Option<(bool,char)>>> = Vec::new();
    let mut symbols: Vec<Vec<bool>> = Vec::new(); 

    let mut row = 0;
    let mut index = 0;
    
    for line in input.lines() {
        numbers.push(Vec::new());
        symbols.push(Vec::new());
        
        line.chars().for_each(|c| {match c.is_numeric() {
            true  => { numbers[row].push(Some((false,c))); symbols[row].push(false);},
            false => { match c {
                '.' => {symbols[row].push(false); numbers[row].push(None);},
                 _  => {symbols[row].push(true); numbers[row].push(None);}
            }}
            };
            index += 1;
        });
        row += 1;
    }

    let mut row: i32 = 0;

    for line in symbols {
        let mut index: i32 = 0;
        for symbol in line {
            if symbol == true {
                for height in -1..=1{
                    for width in -1..=1{
                        match numbers.get_mut((row + height) as usize) {
                            Some(x) => {
                                match x.get_mut((index + width) as usize) {
                                    Some(x) => if let Some(y) = x {y.0 = true},
                                    None => (),
                                };
                            },
                            None => (),
                            };
                        }
                    }
            }
            index +=1
        }
        row += 1;
    }   

    let mut row: i32 = 0;

    for line in numbers.clone() {

        let mut index: i32 = 0;
        for value in line {
            if let Some(x) = value {
                if x.0 == true {
                    'walkback: for i in (-2..=0).rev() {
                        if let Some(x) = numbers.get_mut(row as usize) {
                            match x.get_mut((index+i) as usize) {
                                Some(Some(y)) => y.0 = true,
                                _ => break 'walkback
                            } 
                        }
                    };
                    
                    'walkforwards: for i in 0..=2 {
                        if let Some(x) = numbers.get_mut(row as usize) {
                            match x.get_mut((index+i) as usize) {
                                Some(Some(y)) => y.0 = true,
                                _ => break 'walkforwards
                            } 
                        }
                    };
                }
            }
            index +=1;
        }
        row +=1
    }

    for line in &numbers {
        let line = line.split(|v| {match v { Some(_) => false, None => true }});
        
        for value in line {
            let mut valuebuffer: String = "".to_string();
            for n in value {
                if let Some(x) = n { 
                    if x.0 == true {
                        valuebuffer.push(x.1);
                    }
                }
            }
            match valuebuffer.parse::<u32>() {
                Ok(x) => output.push(x),
                Err(_) => (),
            }
        }
    }

    println!("{:?}", output.iter().sum::<u32>());

}
