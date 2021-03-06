// https://adventofcode.com/2018/day/1
use std::fs::File;
use std::io::{BufReader, Result};
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> Result<()> {
    let file = File::open("data/day1.txt")?;
    let reader = BufReader::new(file);
    let numbers: Vec<i64> = reader.lines().map(|n| {
        let number = n.unwrap();
        if number.starts_with('+') {
            number[1..].parse::<i64>().unwrap()
        } else {
            -number[1..].parse::<i64>().unwrap()
        }
    }).collect();

    let mut frequencies = HashSet::new();
    let mut frequency: i64 = 0;

    for num in numbers.iter().cycle() {
        frequency += num;
        if frequencies.contains(&frequency) {
            println!("repeating frequency is {}", frequency);
            break
        }
        frequencies.insert(frequency);
    }

    Ok(())
}
