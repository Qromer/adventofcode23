
// This is a hell of my own creation.

use std::fs::read_to_string;
use std::option::Option;
fn main() {
    
    let input = read_to_string("input.txt").unwrap();
    
    let mut output: Vec<i32> = Vec::new();
    let mut numbers: Vec<Vec<Option<(bool,char,i32)>>> = Vec::new();
    let mut symbols: Vec<Vec<bool>> = Vec::new(); 

    let mut row = 0;
    let mut index = 0;
    
    for line in input.lines() {
        numbers.push(Vec::new());
        symbols.push(Vec::new());
        
        line.chars().for_each(|c| {match c.is_numeric() {
            true  => { numbers[row].push(Some((false,c,0))); symbols[row].push(false);},
            false => { match c {
                '*' => {symbols[row].push(true); numbers[row].push(None);}
                 _ => {symbols[row].push(false); numbers[row].push(None);},
            }}
            };
            index += 1;
        });
        row += 1;
    }

    let mut row: i32 = 0;
    let mut symbolmatch = 0;
    for line in symbols {
        
        let mut index: i32 = 0;
        for symbol in line {
            if symbol == true {
                for height in -1..=1{
                    for width in -1..=1{
                        match numbers.get_mut((row + height) as usize) {
                            Some(x) => {
                                match x.get_mut((index + width) as usize) {
                                    Some(x) => if let Some(y) = x {y.0 = true; y.2 = symbolmatch},
                                    None => (),
                                };
                            },
                            None => (),
                            };
                        }
                    }
            }
            symbolmatch += 1;
            index +=1;
        }
        row += 1;
    }   

    let mut row: i32 = 0;

    for line in numbers.clone() {

        let mut index: i32 = 0;
        for value in line {
            if let Some(x) = value {
                let symbolmatch = x.2;
                if x.0 == true {
                    'walkback: for i in (-2..=0).rev() {
                        if let Some(x) = numbers.get_mut(row as usize) {
                            match x.get_mut((index+i) as usize) {
                                Some(Some(y)) => {y.0 = true; y.2 = symbolmatch},
                                _ => break 'walkback
                            } 
                        }
                    };
                    
                    'walkforwards: for i in 0..=2 {
                        if let Some(x) = numbers.get_mut(row as usize) {
                            match x.get_mut((index+i) as usize) {
                                Some(Some(y)) => {y.0 = true; y.2 = symbolmatch},
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

    let mut pairs: Vec<(String,i32)> = Vec::new();

    for line in &numbers {
        let line = line.split(|v| {match v { Some(_) => false, None => true }});
        for value in line {
            let mut numberbuffer: String = "".to_string();
            let mut valuebuffer: (String, i32) = ("".to_string(),0);
            for n in value {
                if let Some(x) = n { 
                    if x.0 == true {
                        numberbuffer.push(x.1);
                        valuebuffer.1 = x.2;
                    }
                }
            }
            
            valuebuffer.0 = numberbuffer;

            if valuebuffer.0 != "".to_string() {
                pairs.push(valuebuffer);
            }

        }
    }
    


    pairs.sort_by(|a, b| a.1.cmp(&b.1));

    let mut i = 0;
    while i < pairs.len() {
        if pairs[i].1 == match pairs.get(i+(1 as usize)) {
            Some(x) => x.1,
            None => 0,
        } {
            output.push(pairs[i].0.parse::<i32>().unwrap() * pairs.get(i+1usize).unwrap().0.parse::<i32>().unwrap());
            i+= 1; 
        
        }
        i += 1;
    }

 println!("{:?}", output.iter().sum::<i32>());
}
