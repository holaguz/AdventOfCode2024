use std::{
    collections::VecDeque,
    io::{stdin, BufRead},
};

fn concat(lhs: i64, rhs: i64) -> i64 {

    let mut rhs2 = rhs;
    let mut exp = 0;
    while rhs2 != 0 {
        rhs2 /= 10;
        exp += 1
    }

    lhs * 10i64.pow(exp) + rhs
}

type BinOp = fn(i64, i64) -> i64;

fn check_eq(res: i64, partial: i64, operators: &mut VecDeque<i64>, ops: &[BinOp]) -> bool {
    if partial == res && operators.is_empty() {
        return true;
    }

    if operators.is_empty() || partial >= res {
        return false;
    }

    let front = operators.pop_front().unwrap();

    for op in ops {
        let result = op(partial, front);
        if check_eq(res, result, operators, ops) {
            return true;
        }
    }

    operators.push_front(front);
    false
}

fn main() {
    let lines: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let eqs: Vec<_> = lines
        .iter()
        .map(|l| {
            let mut s = l.split(": ");
            let result = s.next().unwrap().parse::<i64>().unwrap();
            let ops: Vec<_> = s
                .next()
                .unwrap()
                .split(" ")
                .map(|op| op.parse::<i64>().unwrap())
                .collect();
            (result, ops)
        })
        .collect();

    let p1_ops = [|a, b| a + b, |a, b| a * b];

    let p1: i64 = eqs
        .iter()
        .filter(|eq| check_eq(eq.0, 0, &mut eq.1.clone().into(), &p1_ops))
        .map(|eq| eq.0)
        .sum();

    let p2_ops = [|a, b| a + b, |a, b| a * b, concat];
    let p2: i64 = eqs
        .iter()
        .filter(|eq| check_eq(eq.0, 0, &mut eq.1.clone().into(), &p2_ops))
        .map(|eq| eq.0)
        .sum();

    println!("Part 1: {}", p1);

    // 271691107779347
    println!("Part 2: {}", p2);
}
