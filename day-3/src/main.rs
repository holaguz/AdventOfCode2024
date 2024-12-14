use regex::Regex;
use std::io::{stdin, BufRead};

fn parse_ops(s: &str, regex: &Regex) -> Vec<(i32, i32)> {
    regex
        .captures_iter(s)
        .map(|m| {
            let ops = (&m[1], &m[2]);
            (
                ops.0.parse::<i32>().expect("Not a number"),
                ops.1.parse::<i32>().expect("Not a number"),
            )
        })
        .collect()
}

fn main() {
    let op_re = Regex::new(r#"mul\(([\d]{1,3}),([\d]{1,3})\)"#).unwrap();
    let enable_re = Regex::new(r#"(do\(\)|^).*?(don't\(\)|$)"#).unwrap();

    let input: String = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let p1: i32 = parse_ops(input.as_str(), &op_re)
        .iter()
        .map(|op| op.0 * op.1)
        .sum();

    let p2: i32 = enable_re
        .find_iter(input.as_str())
        .map(|m| parse_ops(m.as_str(), &op_re))
        .map(|ops| ops.iter().map(|op| op.0 * op.1).sum::<i32>())
        .sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
