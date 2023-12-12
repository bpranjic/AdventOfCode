use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(line: String) -> u32{
    let first_digit: u32 = line.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last_digit: u32 = line.chars().filter_map(|c| c.to_digit(10)).last().unwrap();
    first_digit * 10 + last_digit
}

fn part2(line: String) -> u32{  
    part1(line
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
    )
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let sum_part1: u32 = reader
        .lines()
        .map(|line| part1(line.unwrap()))
        .sum();
    
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let sum_part2: u32 = reader
        .lines()
        .map(|line| part2(line.unwrap()))
        .sum();
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
    Ok(())
}