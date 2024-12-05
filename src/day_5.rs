use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

const TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

pub fn part_1() -> usize {
    let contents = read_to_string("input/day-5.txt").unwrap();
    let (rules_block, pages_block) = contents.split_once("\n\n").unwrap();
    let rules = rules_block
        .lines()
        .filter_map(|line| line.split_once('|'))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, HashSet<usize>>, (left, right)| {
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                acc.entry(right)
                    .and_modify(|entry| {
                        entry.insert(left);
                    })
                    .or_insert(HashSet::from([left]));
                acc
            },
        );
    pages_block
        .lines()
        .filter_map(|line| {
            let (pages_vec, pages_set) = line.split(',').fold(
                (Vec::new(), HashSet::new()),
                |(mut pages_vec, mut pages_set), cur| {
                    let cur = cur.parse::<usize>().unwrap();
                    pages_vec.push(cur);
                    pages_set.insert(cur);
                    (pages_vec, pages_set)
                },
            );
            for ind in 0..pages_vec.len() {
                if let Some(rules_set) = rules.get(&pages_vec[ind]) {
                    for page in pages_set.intersection(rules_set) {
                        if !pages_vec[..=ind].contains(page) {
                            return None;
                        }
                    }
                }
            }
            Some(pages_vec)
        })
        .map(|seen| seen[seen.len() / 2])
        .sum()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-5.txt").unwrap();
    let (rules_block, pages_block) = contents.split_once("\n\n").unwrap();
    let rules = rules_block
        .lines()
        .filter_map(|line| line.split_once('|'))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, HashSet<usize>>, (left, right)| {
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                acc.entry(right)
                    .and_modify(|entry| {
                        entry.insert(left);
                    })
                    .or_insert(HashSet::from([left]));
                acc
            },
        );
    pages_block
        .lines()
        .filter_map(|line| {
            let (pages_vec, pages_set) = line.split(',').fold(
                (Vec::new(), HashSet::new()),
                |(mut pages_vec, mut pages_set), cur| {
                    let cur = cur.parse::<usize>().unwrap();
                    pages_vec.push(cur);
                    pages_set.insert(cur);
                    (pages_vec, pages_set)
                },
            );
            for ind in 0..pages_vec.len() {
                if let Some(rules_set) = rules.get(&pages_vec[ind]) {
                    for page in pages_set.intersection(rules_set) {
                        if !pages_vec[..=ind].contains(page) {
                            return Some((pages_vec, pages_set));
                        }
                    }
                }
            }
            None
        })
        .map(|(mut pages_vec, pages_set)| {
            let mut ind = 0;
            'outer: while ind < pages_vec.len() {
                if let Some(rules_set) = rules.get(&pages_vec[ind]) {
                    let intersection: Vec<_> = pages_set.intersection(rules_set).collect();
                    let mut page_ind = 0;
                    while page_ind < intersection.len() {
                        if !pages_vec[..=ind].contains(intersection[page_ind]) {
                            let swap_ind = pages_vec[ind + 1..]
                                .iter()
                                .position(|x| x == intersection[page_ind])
                                .unwrap()
                                + ind
                                + 1;
                            pages_vec.swap(ind, swap_ind);
                            continue 'outer;
                        }
                        page_ind += 1;
                    }
                }
                ind += 1;
            }
            pages_vec
        })
        .map(|seen| seen[seen.len() / 2])
        .sum()
}
