use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
extern crate regex;
use regex::Regex;

pub fn question1() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./question1input.txt") {
        for line in lines {
            if let Ok(result) = line {
                sum += find_first_last_digit(&result);
            }
        }
    }
    println!("The answer is {}", sum);
}

fn find_first_last_digit(value: &str) -> i32{
    let r = Regex::new(r"[a-zA-Z]").unwrap();
    let s = r.replace_all(value, "");


    let firsti = 0;
    let lasti = s.len()-1;

    if firsti == lasti {
        let first = s.chars().nth(0).unwrap();   
        return first.to_string().parse().unwrap();
    }

    let first = s.chars().nth(0).unwrap();
    let last = s.chars().nth(s.len()-1).unwrap();

    let mut result: String = first.to_string();
    result.push(last);
    
    result.parse().unwrap()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}