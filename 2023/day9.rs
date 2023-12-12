use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn rec(v: &Vec<i32>) -> i32{
    if v.into_iter().min() == v.into_iter().max(){
        return v[0];
    }
    else{
        let v2 = v
        .into_iter()
        .enumerate()
        .filter(|(i,_)| *i < v.len() - 1)
        .map(|(i, _)| v[i+1] - v[i])
        .collect();
        return v[v.len()-1] + rec(&v2);
    }
}

fn part1(line: String) -> i32{
    rec(&line
        .split_whitespace()
        .into_iter()
        .map(|c| c.to_string()
        .parse().unwrap())
        .collect())
}

fn part2(line: String) -> i32{  
    rec(&line
        .split_whitespace()
        .into_iter()
        .rev()
        .map(|c| c.to_string()
        .parse().unwrap())
        .collect())
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let sum_part1: i32 = reader
        .lines()
        .map(|line| part1(line.unwrap()))
        .sum();
    
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let sum_part2: i32 = reader
        .lines()
        .map(|line| part2(line.unwrap()))
        .sum();
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
    Ok(())
}