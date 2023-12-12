use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

struct CubePool {
    game_id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

pub fn question2() {

    let mut sum = 0;
    if let Ok(lines) = read_lines("./question2input.txt") {
        for line in lines {
            if let Ok(result) = line {
                sum += get_cube_power(&result);
            }
        }
    }
    println!("The answer is {}", sum);
}

fn get_cube_power(game_string: &String) -> i32 {
    let game_cubes = get_cube_pool_from_game_string(game_string);
    return game_cubes.red * game_cubes.green * game_cubes.blue;
}

fn get_cube_pool_from_game_string(game_string: &String) -> CubePool {
    let game_strings = game_string.split([':', ';', ',']);
    let r1 = Regex::new(r"[\d\s]").unwrap();
    let r2 = Regex::new(r"[a-zA-Z\s]").unwrap();
    let mut game_id = 0;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for part in game_strings {
        let sanitised: String = r1.replace_all(part, "").to_string();
        let digit = r2.replace_all(part, "").parse::<i32>().unwrap();
        match sanitised.chars().nth(0).unwrap() {
            'G' => game_id = digit,
            'r' => {
                if red < digit {
                    red = digit
                }
            }
            'g' => {
                if green < digit {
                    green = digit
                }
            }
            'b' => {
                if blue < digit {
                    blue = digit
                }
            }
            _ => panic!(),
        }
    }
    return CubePool {
        game_id,
        red,
        green,
        blue,
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
