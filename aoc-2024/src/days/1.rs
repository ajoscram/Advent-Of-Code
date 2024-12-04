use std::collections::HashMap;

pub const DAY: &str = "1";

pub fn solve(lines: impl Iterator<Item = String>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    
    for line in lines {
        let line_parts: Vec<&str> = line.split("   ").collect();
        left.push(line_parts[0].parse::<i32>().unwrap());
        right.push(line_parts[1].parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let distances_aggregated: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(left_value, right_value)| (left_value - right_value).abs())
        .sum();

    println!("The total distance between lists is {}", distances_aggregated);

    // 2nd star

    let mut left_map = left
        .into_iter()
        .map(|i| (i, 0))
        .collect::<HashMap<_, _>>();

    for i in right {
        left_map
            .entry(i)
            .and_modify(|e| *e += 1);
    }

    let similarity_score: i32 = left_map
        .iter()
        .map(|(left_value, right_occurrences)| left_value * right_occurrences)
        .sum();

    println!("The similarity score is {}", similarity_score);
}