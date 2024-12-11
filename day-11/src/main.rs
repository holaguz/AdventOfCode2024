use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn num_digits(x: u64) -> usize {
    let mut d = 0;
    let mut x = x;
    while x != 0 {
        x /= 10;
        d += 1;
    }

    d
}

fn split(x: u64, offset: usize) -> (u64, u64) {
    let lhs = x / 10_u64.pow(offset as u32);
    let rhs = x - 10_u64.pow(offset as u32) * lhs;
    (lhs, rhs)
}

fn next_stone(stone: u64) -> Vec<u64> {
    let nd = num_digits(stone);
    if stone == 0 {
        vec![1]
    } else if nd % 2 == 0 {
        let splits = split(stone, nd / 2);
        vec![splits.0, splits.1]
    } else {
        vec![stone * 2024]
    }
}

fn main() {
    let input = stdin().lock().lines().next().unwrap().unwrap();
    let stones: Vec<_> = input
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut hash: HashMap<u64, usize> = HashMap::new();
    for stone in stones {
        *hash.entry(stone).or_default() += 1;
    }

    for ix in 0..75 {
        // dbg!(ix, &hash);
        if ix == 25 {
            println!("Part 1: {}", hash.values().sum::<usize>());
        }

        let mut new: HashMap<u64, usize> = HashMap::new();

        for (value, count) in &hash {
            let next = next_stone(*value);
            for new_stone in next {
                *new.entry(new_stone).or_default() += *count;
            }
        }

        hash = new;
    }

    // 65601038650482 too low
    let res: usize = hash.values().sum();
    println!("Part 2: {}", res);
    dbg!(hash.len());
}
