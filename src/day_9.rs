use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST: &str = "2333133121414131402";

// 6430180814248
pub fn part_1() -> usize {
    let contents = read_to_string("input/day-9.txt").unwrap();
    let mut system =
        contents
            .chars()
            .enumerate()
            .fold(Vec::new(), |mut acc: Vec<Option<usize>>, (k, v)| {
                if !v.is_ascii_digit() {
                    return acc;
                }
                let v = v as usize - '0' as usize;
                if k % 2 == 0 {
                    acc.extend(vec![Some(k / 2); v]);
                } else {
                    acc.extend(vec![None; v]);
                }
                acc
            });
    let mut left = 0;
    let mut right = system.len() - 1;
    while left < right {
        while system[left].is_some() {
            left += 1;
        }
        while system[right].is_none() {
            right -= 1;
        }
        if left < right {
            system.swap(left, right);
        }
    }
    let mut ind = 0;
    let mut sum = 0;
    while let Some(val) = system[ind] {
        sum += val * ind;
        ind += 1;
    }
    sum
}

// 6678991767832
pub fn part_2() -> usize {
    let contents = read_to_string("input/day-9.txt").unwrap();
    let mut system = contents.chars().enumerate().fold(
        Vec::new(),
        |mut acc: Vec<(Vec<(usize, usize)>, usize)>, (k, v)| {
            if !v.is_ascii_digit() {
                return acc;
            }
            let v = v as usize - '0' as usize;
            if k % 2 == 0 {
                acc.push((vec![(k / 2, v)], 0));
            } else {
                acc.push((Vec::new(), v));
            }
            acc
        },
    );
    let mut left = 0;
    let mut right = system.len() - 1;
    let mut reset = false;
    loop {
        while left < right && system[left].1 == 0 {
            left += 1;
        }
        if left < right && !system[right].0.is_empty() && system[left].1 >= system[right].0[0].1 {
            let right_block = system[right].0.clone();
            system[left].0.push(right_block[0]);
            system[left].1 -= system[right].0[0].1;
            system[right].1 = system[right].0[0].1;
            system[right].0.clear();
            right = system.len() - 1;
            left = 0;
            reset = false;
        } else if right > 0 {
            right -= 1;
        } else {
            break;
        }

        if left == right {
            right = system.len() - 1;
            if reset {
                break;
            } else if left == system.len() - 1 {
                left = 0;
                reset = true;
            } else {
                left += 1;
            }
        }
    }
    system
        .into_iter()
        .flat_map(|(mut block, size)| {
            if size > 0 {
                block.push((0, size));
            }
            block
        })
        .flat_map(|outer_block| vec![outer_block.0; outer_block.1])
        .enumerate()
        .fold(0, |acc, (ind, id)| acc + ind * id)
}
