use std::collections::HashMap;

pub fn part1() {
    let input = include_str!("input.txt");
    let mut col1: Vec<i32> = vec![];
    let mut col2: Vec<i32> = vec![];
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("Unexpected line: {}", line);
        }

        col1.push(parts[0].parse().unwrap());
        col2.push(parts[1].parse().unwrap());
    }

    col1.sort();
    col2.sort();

    let mut distance: i32 = 0;

    for i in 0..col1.len() {
        distance += (col2[i] - col1[i]).abs();
    }

    println!("Part 1 distance: {}", distance);
}

pub fn part2() {
    let input = include_str!("input.txt");
    let mut col1: Vec<i32> = vec![];
    let mut col2: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if parts.len() != 2 {
            panic!("Unexpected line: {}", line);
        }

        col1.push(parts[0].parse().unwrap());
        *col2.entry(parts[1].parse().unwrap()).or_default() += 1;
    }

    let mut distance: i32 = 0;
    for v in col1 {
        distance += v * *col2.entry(v).or_default();
    }

    println!("Part 2 distance: {}", distance);
}
