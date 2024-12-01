use std::collections::HashMap;

pub fn part1() {
    let input = include_str!("input.txt");
    let mut col1: Vec<u32> = vec![];
    let mut col2: Vec<u32> = vec![];
    for line in input.lines() {
        let parts = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>();
        col1.push(parts[0]);
        col2.push(parts[1]);
    }

    col1.sort();
    col2.sort();

    let distance: u32 = col1
        .into_iter()
        .zip(col2)
        .map(|(v1, v2)| v1.abs_diff(v2))
        .sum();
    println!("Part 1 distance: {}", distance);
}

pub fn part2() {
    let input = include_str!("input.txt");
    let mut col1: Vec<u32> = vec![];
    let mut col2: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let parts = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u32>>();

        col1.push(parts[0]);
        *col2.entry(parts[1]).or_default() += 1;
    }

    let distance: u32 = col1
        .into_iter()
        .map(|v| v * *col2.entry(v).or_default())
        .sum();
    println!("Part 2 distance: {}", distance);
}
