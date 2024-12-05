use std::{collections::HashMap, fs::read_to_string};

const TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

const PATTERN: &[char] = &['X', 'M', 'A', 'S'];

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-4.txt").unwrap();
    let chars: Box<[Box<[char]>]> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut combos = 0;
    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            for x_dir in -1..2 {
                for y_dir in -1..2 {
                    if x_dir == 0 && y_dir == 0 {
                        continue;
                    }
                    combos += recurse_1(&chars, 0, col, row, x_dir, y_dir) as usize;
                }
            }
        }
    }
    combos
}

fn recurse_1(
    chars: &[Box<[char]>],
    pattern_idx: usize,
    pos_x: usize,
    pos_y: usize,
    dir_x: isize,
    dir_y: isize,
) -> bool {
    if pattern_idx == PATTERN.len() {
        return true;
    }
    if chars
        .get(pos_y)
        .and_then(|row| row.get(pos_x))
        .is_some_and(|&char| char == PATTERN[pattern_idx])
    {
        recurse_1(
            chars,
            pattern_idx + 1,
            pos_x.wrapping_add_signed(dir_x),
            pos_y.wrapping_add_signed(dir_y),
            dir_x,
            dir_y,
        )
    } else {
        false
    }
}

const PATTERN_2: &[char] = &['M', 'A', 'S'];


// 1784
pub fn part_2() -> usize {
    let contents = read_to_string("input/day-4.txt").unwrap();
    let chars: Box<[Box<[char]>]> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut combos = 0;
    for row in 0..chars.len() {
        'outer: for col in 0..chars[row].len() {
            if chars
                .get(row)
                .and_then(|row| row.get(col))
                .is_none_or(|&char| char != PATTERN_2[1])
            {
                continue;
            }
            let mut matches: Vec<char> = Vec::with_capacity(4);
            for x_dir in [-1, 1] {
                for y_dir in [-1, 1] {
                    let row = row.wrapping_add_signed(y_dir);
                    let col = col.wrapping_add_signed(x_dir);
                    if let Some(&char) = chars.get(row).and_then(|row| row.get(col)) {
                        for idx in [0, 2] {
                            if char == PATTERN_2[idx] {
                                matches.push(char);
                            }
                        }
                    } else {
                        continue 'outer;
                    }
                }
            }
            for i in 0..2 {
                match (matches.get(i), matches.get(3-i)) {
                    (Some(&left), Some(&right)) => {
                        if left == right {
                            continue 'outer;
                        }
                    }
                    _ => continue 'outer,
                }
            }
            combos += 1;
        }
    }
    combos
}

