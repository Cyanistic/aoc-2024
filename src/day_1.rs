use std::{collections::HashMap, fs::read_to_string};

pub fn part_1() -> u32 {
    let contents = read_to_string("input/day-1.txt").unwrap();
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = contents
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

pub fn part_2() -> i32 {
    let contents = read_to_string("input/day-1.txt").unwrap();
    let (left, right): (Vec<i32>, HashMap<i32, i32>) = contents
        .lines()
        .map(|line| {
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse::<i32>().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold(
            (Vec::new(), HashMap::new()),
            |(mut left_acc, mut right_acc), (left, right)| {
                left_acc.push(left);
                right_acc
                    .entry(right)
                    .and_modify(|entry| *entry += 1)
                    .or_insert(1);
                (left_acc, right_acc)
            },
        );

    left.into_iter()
        .map(|left| left * right.get(&left).unwrap_or(&0))
        .sum()
}
