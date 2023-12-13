extern crate regex;
use regex::Regex;
use std::collections::HashMap;

pub fn question1(input: Vec<String>) {
    let mut sum = 0;
    let digits_map: HashMap<String, i32> = create_digits_map();
    for line in input {
        sum += find_first_last_digit(&line, &digits_map);
    }
    println!("The answer is {}", sum);
}

fn create_digits_map() -> HashMap<String, i32> {
    let mut string_digits = HashMap::new();
    string_digits.insert(String::from("zero"), 0);
    string_digits.insert(String::from("one"), 1);
    string_digits.insert(String::from("two"), 2);
    string_digits.insert(String::from("three"), 3);
    string_digits.insert(String::from("four"), 4);
    string_digits.insert(String::from("five"), 5);
    string_digits.insert(String::from("six"), 6);
    string_digits.insert(String::from("seven"), 7);
    string_digits.insert(String::from("eight"), 8);
    string_digits.insert(String::from("nine"), 9);

    string_digits
}

fn find_first_last_digit(value: &str, map: &HashMap<String, i32>) -> i32 {
    //println!("Input string is {}", value);

    // Initial string sanitisation to remove digits and make lowercase
    let r = Regex::new(r"[0-9]").unwrap();
    let s = r.replace_all(value, "").to_lowercase();

    //println!("S string is {}", s);

    let mut current = String::new();
    let mut first = None::<i32>;
    let mut last = None::<i32>;
    let mut string_i = 0;
    let mut key_i = 0;
    let mut added: bool;
    while string_i < s.len() {
        //println!("i is {}", string_i.to_string());
        //println!("k is {}", key_i.to_string());
        added = false;

        for (key, _value) in map {
            let s_char = s.chars().nth(string_i).unwrap();
            if key_i < key.len() {
                let key_char = key.chars().nth(key_i).unwrap();
                if s_char == key_char {
                    current.push(s_char);
                    added = true;
                    break;
                }
            }
        }

        if added == false {
            if !current.is_empty() {
                key_i = 0;
                current.clear();
            } else {
                string_i += 1;
                key_i = 0;
            }
            continue;
        }

        //println!("Current is {}", current);
        let keyvalue = map.get(&current);
        if keyvalue != None {
            if first == None {
                first = keyvalue.copied();
            }
            last = keyvalue.copied();

            current.clear();
            key_i = 0;
        } else {
            key_i += 1;
            string_i += 1;
        }
    }

    if first == None {
        println!("No digits found for string {} where s is {}", value, s);
        return 0;
    }

    println!(
        "Input string is {}. S is {}. Numbers are {} and {}",
        value,
        s,
        first.unwrap().to_string(),
        last.unwrap().to_string()
    );
    (first.unwrap() * 10) + last.unwrap()
}
