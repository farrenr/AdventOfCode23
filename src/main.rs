use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod question1;
mod question2;

fn main() {
    let lines = read_lines("./question2input.txt").unwrap();
    question2::question2(lines);
}

fn read_lines<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut lines: Vec<String> = vec![];
    for line in buf.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(err) => panic!("Could not read line. {}", err),
        }
    }
    Ok(lines)
}
