use std::collections::{BinaryHeap, HashMap};
use std::io::{stdin, BufRead};

fn main() {
    let (lhs, rhs): (BinaryHeap<i32>, BinaryHeap<i32>) = stdin()
        .lock()
        .lines()
        .filter_map(|l| {
            let line = l.unwrap();
            line.split_once("   ")
                .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        })
        .collect();

    let part_1: i32 = lhs
        .clone()
        .into_sorted_vec()
        .iter()
        .zip(rhs.clone().into_sorted_vec().iter())
        .map(|(lhs, rhs)| (lhs - rhs).abs())
        .sum();

    println!("Part 1: {}", part_1);

    /* */

    let mut rhs_map = HashMap::<i32, i32>::new();
    rhs.iter()
        .for_each(|&k| *rhs_map.entry(k).or_default() += 1);

    let part_2 = lhs
        .iter()
        .map(|l| l * rhs_map.get(l).unwrap_or(&0))
        .sum::<i32>();

    println!("Part 2: {}", part_2);
}
