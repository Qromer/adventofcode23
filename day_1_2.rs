use std::fs::read_to_string;

struct StrNumConv<'a> {
    pattern: &'a str,
    value: char,
}

const CONVERSIONS: [StrNumConv; 9] = [
StrNumConv {pattern: "one", value: '1'},
StrNumConv {pattern: "two", value: '2'},
StrNumConv {pattern: "three", value: '3'},
StrNumConv {pattern: "four", value: '4'},
StrNumConv {pattern: "five", value: '5'},
StrNumConv {pattern: "six", value: '6'},
StrNumConv {pattern: "seven", value: '7'},
StrNumConv {pattern: "eight", value: '8'},
StrNumConv {pattern: "nine", value: '9'},
];


fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut output: Vec<u32> = Vec::new(); 

    input.lines()
    .for_each(|line| { 

        let mut result: String = String::new();
        let line = line.to_string();
        let mut outputline = line.clone();

        for conv in CONVERSIONS {
            let indices = line.match_indices(conv.pattern).map(|x| x.0 ).collect::<Vec<usize>>();   
            for index in indices {
                outputline.insert(index + conv.pattern.len() , conv.value);
            };
        };
        
        let mut chars = outputline.chars()
        .filter(|c| c.to_digit(10).is_some());

        if let Some(x) = chars.next() {
            result.push(x);
        };
        
        match chars.last() {
            Some(x) => {result.push(x)},
            None => {result = result.clone() + &result},
        }  

        output.push(result.parse::<u32>().unwrap());

    });

    println!{"{}",output.iter().sum::<u32>()};


}
