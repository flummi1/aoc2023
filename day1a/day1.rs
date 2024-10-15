use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut calibration_list: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines.flatten() {
            calibration_list.push(get_calibration(&line));
        }
    }

    println!("{}", calibration_list.iter().sum::<i32>())
}

fn get_calibration(input: &str) -> i32 {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_numeric() {
            result.push(c);
            break;
        }
    }

    for c in input.chars().rev() {
        if c.is_numeric() {
            result.push(c);
            break;
        }
    }

    return result.parse::<i32>().unwrap_or(0);
}
