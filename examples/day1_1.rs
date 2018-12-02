// https://adventofcode.com/2018/day/1
use std::fs::File;
use std::io::{BufReader, Result};
use std::io::prelude::*;

fn main() -> Result<()> {
    let file = File::open("data/day1.txt")?;
    let reader = BufReader::new(file);

    let frequency: i64 = reader.lines()
        .map(|n| {
            let number = n.unwrap();
            if number.starts_with('+') {
                number[1..].parse::<i64>().unwrap()
            } else {
                -number[1..].parse::<i64>().unwrap()
            }
        })
        .sum();
    println!("{:?}", frequency);
    Ok(())
}
