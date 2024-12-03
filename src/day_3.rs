use std::{char, fs::read_to_string, ops::ControlFlow, thread::park};

const TEST: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

const PATTERN: &[char] = &['m', 'u', 'l', '(', '!', ',', '!', ')'];

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-3.txt").unwrap();
    let mut pattern_idx: usize = 0;
    let mut sum = 0;
    let mut left = String::new();
    let mut right = String::new();
    let mut in_left = true;
    let chars: Vec<char> = contents.chars().collect();
    for char_idx in 0..chars.len() {
        let char = chars[char_idx];
        if char == PATTERN[pattern_idx] || (char.is_ascii_digit() && PATTERN[pattern_idx] == '!') {
            match PATTERN[pattern_idx] {
                '!' => {
                    if in_left {
                        left.push(char);
                    } else {
                        right.push(char);
                    }
                    if chars[char_idx + 1].is_ascii_digit() {
                        pattern_idx -= 1;
                    }
                }
                ',' => in_left = false,
                _ => (),
            }
            pattern_idx += 1;
        } else {
            pattern_idx = 0;
            left.clear();
            right.clear();
            in_left = true;
            continue;
        }

        if pattern_idx == PATTERN.len() {
            sum += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
            pattern_idx = 0;
            left.clear();
            right.clear();
            in_left = true;
        }
    }
    sum
}

const DO: &[char] = &['d', 'o', '(', ')'];
const DONT: &[char] = &['d', 'o', 'n', '\'', 't', '(', ')'];
const TEST2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-3.txt").unwrap();
    let mut pattern_idx: usize = 0;
    let mut do_idx: usize = 0;
    let mut dont_idx: usize = 0;
    let mut sum = 0;
    let mut left = String::new();
    let mut right = String::new();
    let mut in_left = true;
    let mut enabled = true;
    let chars: Vec<char> = contents.chars().collect();
    for char_idx in 0..chars.len() {
        let char = chars[char_idx];

        if char == DO[do_idx] {
            do_idx += 1;
        } else {
            do_idx = 0;
        }

        if char == DONT[dont_idx] {
            dont_idx += 1;
        } else {
            dont_idx = 0;
        }

        if char == PATTERN[pattern_idx] || (char.is_ascii_digit() && PATTERN[pattern_idx] == '!') {
            match PATTERN[pattern_idx] {
                '!' => {
                    if in_left {
                        left.push(char);
                    } else {
                        right.push(char);
                    }
                    if chars[char_idx + 1].is_ascii_digit() {
                        pattern_idx -= 1;
                    }
                }
                ',' => in_left = false,
                _ => (),
            }
            pattern_idx += 1;
        } else {
            pattern_idx = 0;
            left.clear();
            right.clear();
            in_left = true;
        }

        if do_idx == DO.len() {
            enabled = true;
            do_idx = 0;
        }

        if dont_idx == DONT.len() {
            enabled = false;
            dont_idx = 0;
        }

        if pattern_idx == PATTERN.len() {
            if enabled {
                sum += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
            }
            pattern_idx = 0;
            left.clear();
            right.clear();
            in_left = true;
        }
    }
    sum
}
