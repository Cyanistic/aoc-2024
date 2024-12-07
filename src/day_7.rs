use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-7.txt").unwrap();
    contents
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(test, values)| {
            let test: usize = test.parse().unwrap();
            let values: Vec<usize> = values
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .rev()
                .collect();
            let recursed = recurse_1(&values);
            if recursed.contains(&test) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

fn recurse_1(buf: &[usize]) -> HashSet<usize> {
    if buf.len() == 1 {
        return HashSet::from([buf[0]]);
    }
    let mut values = HashSet::new();
    let recursed = recurse_1(&buf[1..]);
    for op in ['+', '*'] {
        match op {
            '+' => {
                for item in &recursed {
                    values.insert(buf[0] + item);
                }
            }
            '*' => {
                for item in &recursed {
                    values.insert(buf[0] * item);
                }
            }
            _ => unreachable!(),
        }
    }
    values
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-7.txt").unwrap();
    contents
        .lines()
        .filter_map(|line| line.split_once(':'))
        .filter_map(|(test, values)| {
            let test: usize = test.parse().unwrap();
            let values: Vec<usize> = values
                .trim()
                .split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .rev()
                .collect();
            let recursed = recurse_2(&values);
            if recursed.contains(&test) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

fn recurse_2(buf: &[usize]) -> HashSet<usize> {
    if buf.len() == 1 {
        return HashSet::from([buf[0]]);
    }
    let mut values = HashSet::new();
    let recursed = recurse_2(&buf[1..]);
    for op in ['+', '*', '|'] {
        match op {
            '+' => {
                for item in &recursed {
                    values.insert(buf[0] + item);
                }
            }
            '*' => {
                for item in &recursed {
                    values.insert(buf[0] * item);
                }
            }
            '|' => {
                for item in &recursed {
                    values.insert(format!("{}{}", item, buf[0]).parse().unwrap());
                }
            }
            _ => unreachable!(),
        }
    }
    values
}
