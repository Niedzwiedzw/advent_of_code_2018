// https://adventofcode.com/2018/day/2
use std::fs::File;
use std::io::prelude::*;
use std::io::{ BufReader, Result };

fn diff(first: &str, second: &str) -> Vec<char> {
    first.bytes()
        .zip(second.bytes())
        .filter(|c| c.0 != c.1)
        .map(|pair| pair.0 as char)
        .collect()
}

fn main() -> Result<()> {
    let file = File::open("data/day2.txt")?;
    let mut lines: Vec<_> = BufReader::new(file).lines()
        .map(|l| l.unwrap())
        .collect();

    lines.sort();

    for (first, second) in lines.iter().zip(lines.iter().skip(1)) {
        let difference = diff(first, second);
        if difference.len() == 1 {
            let common: String = first.chars().filter(|e| *e != difference[0]).collect();
            println!("checksum is {:?}", common);
        }
    }

    Ok(())
}
