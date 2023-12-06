use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

fn number_of_matches(line: String) -> u32{
    let v: Vec<_> = line.split(":").collect();
    let numbers: Vec<_> = v[1].split("|").collect();
    let winners: HashSet<u32> = numbers[0]
        .split_whitespace()
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    let picks: HashSet<u32> = numbers[1]
        .split_whitespace()
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect();
    let intersection: HashSet<_> = winners.intersection(&picks).collect();
    intersection.len().try_into().unwrap()
}

fn sum_of_children(id: usize, occurences: Vec<u32>) -> u32{
    let mut sum: u32 = occurences[id];
    for i in (id+1).try_into().unwrap()..(u32::try_from(id+1).unwrap() + sum).try_into().unwrap(){
        sum += sum_of_children(i, occurences.clone());
    }
    sum
}

fn part1(line: String) -> u32{
    let count: u32 = number_of_matches(line);
    if count == 0{
        return 0;
    }
    u32::pow(2, count-1)
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
    let occurences: Vec<u32> = reader
        .lines()
        .map(|line| number_of_matches(line.unwrap()))
        .collect();
    let sum_part2: u32 = occurences.clone()
        .into_iter()
        .enumerate()
        .map(|(i, x)| sum_of_children(i, occurences.clone()))
        .fold(occurences.clone().len().try_into().unwrap(), |acc, num| acc + num);
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
    Ok(())
}