use std::{collections::HashMap, fs::read_to_string};

const TEST: &str = "125 17";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-11.txt").unwrap();
    let mut stones: Vec<usize> = contents
        .split_ascii_whitespace()
        .map(|stone| stone.parse().unwrap())
        .collect();
    for i in 0..25 {
        let mut temp_stones = Vec::new();
        for stone in stones {
            match stone {
                _ if stone == 0 => {
                    temp_stones.push(1);
                }
                _ if stone.to_string().len() % 2 == 0 => {
                    let stone_str = stone.to_string();
                    let len = stone_str.len();
                    temp_stones.push(stone_str[..len / 2].parse().unwrap());
                    temp_stones.push(stone_str[len / 2..].parse().unwrap());
                }
                _ => {
                    temp_stones.push(stone * 2024);
                }
            }
        }
        stones = temp_stones;
    }
    stones.len()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-11.txt").unwrap();
    let mut stones: HashMap<usize, usize> = contents
        .split_ascii_whitespace()
        .map(|stone| stone.parse().unwrap())
        .fold(HashMap::new(), |mut acc, stone| {
            acc.entry(stone)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
            acc
        });
    for _ in 0..75 {
        stones = stones.iter().fold(
            HashMap::new(),
            |mut temp_stones: HashMap<usize, usize>, (&stone, &count)| {
                match stone {
                    _ if stone == 0 => {
                        temp_stones
                            .entry(1)
                            .and_modify(|entry| *entry += count)
                            .or_insert(count);
                    }
                    _ if stone.to_string().len() % 2 == 0 => {
                        let stone_str = stone.to_string();
                        let len = stone_str.len();
                        let left = stone_str[..len / 2].parse().unwrap();
                        let right = stone_str[len / 2..].parse().unwrap();
                        temp_stones
                            .entry(left)
                            .and_modify(|entry| *entry += count)
                            .or_insert(count);
                        temp_stones
                            .entry(right)
                            .and_modify(|entry| *entry += count)
                            .or_insert(count);
                    }
                    _ => {
                        temp_stones
                            .entry(stone * 2024)
                            .and_modify(|entry| *entry += count)
                            .or_insert(count);
                    }
                }
                temp_stones
            },
        );
    }
    stones.into_values().sum()
}
