use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-10.txt").unwrap();
    let chars: Box<[Box<[u8]>]> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    if char.is_ascii_digit() {
                        char as u8 - b'0'
                    } else {
                        10
                    }
                })
                .collect()
        })
        .collect();
    let mut paths: HashMap<(usize, usize), HashSet<_>> = HashMap::new();
    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            recurse_1(&chars, &mut paths, 0, row, col, (row, col));
        }
    }
    paths.into_values().map(|path| path.len()).sum()
}

fn recurse_1(
    chars: &[Box<[u8]>],
    cache: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
    expected: u8,
    row: usize,
    col: usize,
    head: (usize, usize),
) {
    if !chars
        .get(row)
        .and_then(|row| row.get(col))
        .is_some_and(|&char| char == expected)
    {
        return;
    }

    if expected == 9 {
        cache
            .entry(head)
            .and_modify(|entry| {
                entry.insert((row, col));
            })
            .or_insert(HashSet::from([(row, col)]));
    } else {
        for dy in -1..2 {
            for dx in -1..2 {
                if dy == dx || dy == -dx {
                    continue;
                }
                recurse_1(
                    chars,
                    cache,
                    expected + 1,
                    row.wrapping_add_signed(dy),
                    col.wrapping_add_signed(dx),
                    head,
                );
            }
        }
    }
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-10.txt").unwrap();
    let chars: Box<[Box<[u8]>]> = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    if char.is_ascii_digit() {
                        char as u8 - b'0'
                    } else {
                        10
                    }
                })
                .collect()
        })
        .collect();
    let mut paths: HashMap<(usize, usize), usize> = HashMap::new();
    for row in 0..chars.len() {
        for col in 0..chars[row].len() {
            recurse_2(&chars, &mut paths, 0, row, col, (row, col));
        }
    }
    paths.into_values().sum()
}

fn recurse_2(
    chars: &[Box<[u8]>],
    cache: &mut HashMap<(usize, usize), usize>,
    expected: u8,
    row: usize,
    col: usize,
    head: (usize, usize),
) {
    if !chars
        .get(row)
        .and_then(|row| row.get(col))
        .is_some_and(|&char| char == expected)
    {
        return;
    }

    if expected == 9 {
        cache
            .entry(head)
            .and_modify(|entry| *entry += 1)
            .or_insert(1);
    } else {
        for dy in -1..2 {
            for dx in -1..2 {
                if dy == dx || dy == -dx {
                    continue;
                }
                recurse_2(
                    chars,
                    cache,
                    expected + 1,
                    row.wrapping_add_signed(dy),
                    col.wrapping_add_signed(dx),
                    head,
                );
            }
        }
    }
}
