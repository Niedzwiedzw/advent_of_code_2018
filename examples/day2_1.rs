// https://adventofcode.com/2018/day/2
use std::fs::File;
use std::io::prelude::*;
use std::io::{ BufReader, Result };
use std::collections::HashMap;

fn to_dict(line: String) -> HashMap<char, u8> {
    let mut dict: HashMap<char, u8> = HashMap::new();
    for c in line.bytes() {
        let entry = dict.entry(c as char).or_insert(0);
        *entry += 1;
    }

    dict
}

fn has_two(dict: &HashMap<char, u8>) -> bool {
    dict.values().any(|e| *e == 2)
}

fn has_three(dict: &HashMap<char, u8>) -> bool {
    dict.values().any(|e| *e == 3)
}

fn main() -> Result<()> {
    let file = File::open("data/day2.txt")?;
    let reader = BufReader::new(file);
    let dicts: Vec<_> = reader.lines().map(|l| to_dict(l.unwrap())).collect();

    let twos = dicts.iter().filter(|dict| has_two(&dict)).count();
    let threes = dicts.iter().filter(|dict| has_three(&dict)).count();

    println!("checksum is {}", twos*threes);
    Ok(())
}
