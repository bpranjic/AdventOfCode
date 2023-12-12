use std::collections::HashMap;
use std::fs;
use std::io::{self};

fn part1(s: &str) -> u64{
    let v: Vec<&str> = s.split_whitespace().collect();
    let springs = v[0];
    let constellation: Vec<u64> = v[1].split(",")
        .into_iter()
        .map(|n| 
            n.parse().unwrap())
        .collect(); 
    let mut dp: HashMap<(u64, u64, u64), u64> = HashMap::new();
    dp_step(&mut dp, 0, 0, 0, springs, &constellation)
}

fn part2(i: usize, s: &str) -> u64{
    println!("Part 2, Line {}", i);
    let v: Vec<&str> = s.split_whitespace().collect();
    let mut springs: String = "".to_string();
    let mut constellation_string: String = "".to_string();
    for i in 0..5{
        springs.push_str(v[0]);
        constellation_string.push_str(v[1]);
        if i < 4{
            springs.push_str("?");
            constellation_string.push_str(",")
        }
    }
    let constellation: Vec<u64> = constellation_string.split(",")
        .into_iter()
        .map(|n| 
            n.parse().unwrap())
        .collect(); 
    let mut dp: HashMap<(u64, u64, u64), u64> = HashMap::new();
    dp_step(&mut dp, 0, 0, 0, &springs, &constellation)
}

fn dp_step(dp: &mut HashMap<(u64, u64, u64), u64>, curr: u64, si: u64, ci: u64, springs: &str, constellation: &Vec<u64>) -> u64{
    if dp.contains_key(&(curr, si, ci)){
        return dp[&(curr, si, ci)];
    }
    if si == springs.len() as u64{
        if ci == constellation.len() as u64{
            return 1;
        }
        else if ci == constellation.len() as u64 - 1 && curr == constellation[ci as usize]{
            return 1;
        }
        else{
            return 0;
        }
    }
    let mut sum = 0;
    for c in ['#','.']{
        if springs.chars().collect::<Vec<char>>()[si as usize] == c 
            || springs.chars().collect::<Vec<char>>()[si as usize] == '?'{
                if c == '.'{
                    if curr == 0{
                        sum += dp_step(dp, 0, si+1, ci, springs, constellation);
                    }
                    else if curr == constellation[ci as usize] && ci < constellation.len() as u64{
                        sum += dp_step(dp, 0, si+1, ci+1, springs, constellation)
                    }
                    else{
                        continue;
                    }
                }
                else{
                    if ci < constellation.len() as u64 && curr < constellation[ci as usize]{
                        sum += dp_step(dp, curr+1, si+1, ci, springs, constellation)
                    }
                }
        }
    }
    dp.insert((curr, si, ci), sum);
    sum
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();
    let sum_part1: u64 = lines
        .into_iter()
        .map(|line| part1(line))
        .sum();
    println!("Part 1: {}", sum_part1);
    let input = fs::read_to_string("input.txt")?;
    let lines: Vec<&str> = input.split("\n").collect();
    let sum_part2: u64 = lines
        .into_iter()
        .enumerate()
        .map(|(i, line)| part2(i,line))
        .sum();
    println!("Part 2: {}", sum_part2);
    Ok(())
}