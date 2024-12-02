use std::{fs::read_to_string, ops::ControlFlow};

const TEST: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
pub fn part_1() -> usize {
    let contents = read_to_string("input/day-2.txt").unwrap();
    contents
        .lines()
        .filter(|line| {
            matches!(
                line.split_ascii_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<usize>>()
                    .windows(2)
                    .try_fold(None, |acc, nums| {
                        let increasing = nums[0] < nums[1];
                        let diff = nums[0].abs_diff(nums[1]);
                        if acc.is_some_and(|x| x != increasing) || !matches!(diff, 1..=3) {
                            return ControlFlow::Break(acc);
                        }
                        ControlFlow::Continue(Some(increasing))
                    }),
                ControlFlow::Continue(_)
            )
        })
        .count()
}

pub fn part_2() -> usize {
    let contents = read_to_string("input/day-2.txt").unwrap();
    contents
        .lines()
        .filter(|line| {
            let arr = line
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>();
            (0..arr.len())
                .map(|i| {
                    [&arr[0..i], &arr[i + 1..]]
                        .concat()
                        .windows(2)
                        .try_fold(None, |acc, nums| {
                            let increasing = nums[0] < nums[1];
                            let diff = nums[0].abs_diff(nums[1]);
                            if acc.is_some_and(|x| x != increasing) || !matches!(diff, 1..=3) {
                                return ControlFlow::Break(acc);
                            }
                            ControlFlow::Continue(Some(increasing))
                        })
                })
                .any(|arr| matches!(arr, ControlFlow::Continue(_)))
        })
        .count()
}
