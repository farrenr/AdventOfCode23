use array2d::{Array2D, Error};
use regex::Regex;

pub fn question3(input: Vec<String>) {
    let grid = build_grid(input);
    let answer = find_sum_part_numbers(grid);

    println!("The answer is {}", answer);
}

fn find_sum_part_numbers(grid: Array2D<char>) -> i32 {
    todo!()
}

fn try_get_at_index(grid: Array2D<char>, row:usize, column:usize) -> char{
    let mut result = '.';
    if let option = grid.get(row, column) {
        match option {
            Some(ch) => result = *ch,
            _ => '.',
        }
    }
    result
}

fn build_grid(input: Vec<String>) -> Array2D<char> {
    let mut rows: Vec<Vec<char>> = vec![];
    for row in input {
        rows.push(row.chars().collect());
    }
    Array2D::from_rows(&rows).unwrap()
}