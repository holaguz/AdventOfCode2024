use std::borrow::BorrowMut;
use std::collections::HashSet;
use std::io::{stdin, BufRead};

fn is_safe(v: &Vec<i32>) -> bool {
    match v.iter().all(|&x| x > 0) {
        true => v.iter().all(|&x| x >= 1 && x <= 3),
        false => match v.iter().all(|&x| x < 0) {
            true => v.iter().all(|&x| x >= -3 && x <= -1),
            false => false,
        },
    }
}

fn rebuild(diff: &Vec<i32>) -> Vec<i32> {
    let mut res = vec![0i32];

    diff.iter().for_each(|v| {
        res.push(res.last().unwrap() + v);
    });

    res
}

fn get_diff(v: &Vec<i32>) -> Vec<i32> {
    v.windows(2).map(|w| w[1] as i32 - w[0] as i32).collect()
}

fn main() {
    let levels: Vec<Vec<i32>> = stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut safe = 0;
    let mut fixable = 0;

    for level in levels {
        let diff = get_diff(&level);
        if is_safe(&diff) {
            safe += 1;
        } else {
            for ix in 0..level.len() {
                let modified: Vec<_> = level
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != ix)
                    .map(|(_, v)| *v)
                    .collect();

                if is_safe(&get_diff(&modified)) {
                    fixable += 1;
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", safe);
    println!("Part 2: {}", safe + fixable);
}
