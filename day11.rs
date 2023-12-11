use std::fs;
use std::io::{self};

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len()-1)
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn distances(galaxies: Vec<Vec<i64>>, rows: Vec<i64>, cols: Vec<i64>, gap: i64) -> i64{
    let mut sum = 0;
    for i in 0..galaxies.len(){
        for j in i+1..galaxies.len(){
            let x1 = galaxies[i][0];
            let x2 = galaxies[j][0];
            let y1 = galaxies[i][1];
            let y2 = galaxies[j][1];
            let mut xdiff = (x1-x2).abs();
            let mut ydiff = (y1-y2).abs();
            for row in rows.clone(){
                if (x1..x2).contains(&row) || (x2..x1).contains(&row){
                    xdiff += gap;
                }
            }
            for col in cols.clone(){
                if (y1..y2).contains(&col) || (y2..y1).contains(&col){
                    ydiff += gap;
                }
            }
            sum += xdiff + ydiff;
        }
    }
    sum
}



fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    let v: Vec<&str> = input.split("\n").collect();
    let mut lines:Vec<Vec<char>> = vec![];
    let mut galaxies: Vec<Vec<i64>> = vec![];
    let mut empty_rows: Vec<i64> = vec![];
    let mut empty_cols: Vec<i64> = vec![];
    for (i, line) in v.into_iter().enumerate(){
        let mut tmp: Vec<char> = vec![];
        let mut count = 0;
        for (j, c) in line.to_string().chars().enumerate(){
            tmp.push(c);
            if c == '#'{
                galaxies.push(vec![i as i64, j as i64]);
                count += 1;
            }
        }
        if count == 0{
            empty_rows.push(i as i64);
        }
        lines.push(tmp);
    }
    let lines_transpose: Vec<Vec<char>> = transpose(lines);
    for (i, line) in lines_transpose.into_iter().enumerate(){
        let mut count = 0;
        for c in line{
            if c == '#'{
                count += 1;
            }
        }
        if count == 0{
            empty_cols.push(i as i64);
        }
    }
    let sum_part1 = distances(galaxies.clone(), empty_rows.clone(), empty_cols.clone(), 1);
    let sum_part2 = distances(galaxies.clone(), empty_rows.clone(), empty_cols.clone(), 999999);
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
    Ok(())
}