use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-8.txt").unwrap();
    let char_positions = contents
        .lines()
        .enumerate()
        .flat_map(|(row, line)| ((row..row + 1).cycle().zip(line.chars().enumerate())))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<char, Vec<(usize, usize)>>, (row, (col, char))| {
                if char != '.' {
                    acc.entry(char)
                        .and_modify(|entry| {
                            entry.push((row, col));
                        })
                        .or_insert(vec![(row, col)]);
                }
                acc
            },
        );

    let cols = contents.split_once('\n').unwrap().0.len() - 1;
    let rows = contents.lines().count() - 1;
    let mut unique = HashSet::new();
    for positions in char_positions.values() {
        for i in 0..positions.len() {
            let cur = positions[i];
            let remaining = [&positions[..i], &positions[i + 1..]].concat();
            for other in remaining {
                let row = (2 * cur.0).wrapping_sub(other.0);
                let col = (2 * cur.1).wrapping_sub(other.1);
                if row <= cols && col <= rows {
                    unique.insert((row, col));
                }
            }
        }
    }
    unique.len()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-8.txt").unwrap();
    let char_positions = contents
        .lines()
        .enumerate()
        .flat_map(|(row, line)| ((row..row + 1).cycle().zip(line.chars().enumerate())))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<char, Vec<(usize, usize)>>, (row, (col, char))| {
                if char != '.' {
                    acc.entry(char)
                        .and_modify(|entry| {
                            entry.push((row, col));
                        })
                        .or_insert(vec![(row, col)]);
                }
                acc
            },
        );

    let cols = contents.split_once('\n').unwrap().0.len() - 1;
    let rows = contents.lines().count() - 1;
    let mut unique = HashSet::new();
    for positions in char_positions.values() {
        for i in 0..positions.len() {
            let cur = positions[i];
            let remaining = [&positions[..i], &positions[i + 1..]].concat();
            unique.insert(cur);
            for other in remaining {
                let x_diff = cur.0 as isize - other.0 as isize;
                let y_diff = cur.1 as isize - other.1 as isize;
                let mut row = cur.0.wrapping_add_signed(x_diff);
                let mut col = cur.1.wrapping_add_signed(y_diff);
                while row <= cols && col <= rows {
                    unique.insert((row, col));
                    row = row.wrapping_add_signed(x_diff);
                    col = col.wrapping_add_signed(y_diff);
                }
            }
        }
    }
    unique.len()
}
