fn f(c: u64, x: u64) -> f64{
    c as f64 - 2.0*(0.5 * (c as f64 - ((c*c - 4*x) as f64).sqrt())).ceil() + 1.0
}

fn main() {
    let times: Vec<u64> = vec![38, 94, 79, 70];
    let distances: Vec<u64> = vec![241, 1549, 1074, 1091];
    let sum_part1: f64 = (0..times.len())
        .into_iter()
        .enumerate()
        .map(|(i, _)| f(times[i], distances[i]))
        .fold(1.0, |acc, num| acc*num);
    let time: String = times
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");
    let distance: String = distances
        .into_iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");
    println!("time: {}, distance: {}", time, distance);
    let sum_part2: f64 = f(time.parse().unwrap(), distance.parse().unwrap());
    
    println!("Part 1: {}", sum_part1);
    println!("Part 2: {}", sum_part2);
}