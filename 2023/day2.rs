use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1_evaluate_draw(draw: String) -> bool{
    let v: Vec<_> = draw.split_whitespace().collect();
    let count: u32 = v[0].parse().unwrap();
    let color: &str = v[1];
    match (count, color){
        (1..=12, "red") => {true}
        (1..=13, "green") => {true}
        (1..=14, "blue") => {true}
        (_, _) => {false}
    }
}

fn part1_evaluate_bag(bag: String) -> bool{
    let v: Vec<_> = bag.split(",").collect();
    v.into_iter().fold(true, |acc, draw| acc & part1_evaluate_draw(draw.to_string()))
}

fn part1(line: String) -> u32{
    let v: Vec<_> = line.split(":").collect();
    let id: u32 = v[0].split(" ").collect::<Vec<_>>()[1].parse().unwrap();
    let bags: Vec<_> = v[1].split(";").collect();
    let b: bool = bags.into_iter().fold(true, |acc, bag| acc & part1_evaluate_bag(bag.to_string()));
    if b {id} else {0}
}

fn part2_find_max(draws: &Vec<&str>, color: &str) -> u32{
    draws.iter().filter(|&draw| draw.contains(color))
        .into_iter()
        .map(|&s| s.split_whitespace().next().unwrap().parse::<u32>().unwrap())
        .max().unwrap()
}

fn part2(line: String) -> u32{
    let v: Vec<_> = line.split(":").collect();
    let draws: Vec<_> = v[1].split([',', ';']).collect();
    part2_find_max(&draws, "red") * part2_find_max(&draws, "green") * part2_find_max(&draws, "blue")
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