use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, BufRead},
};

fn part1(
    obstacles: HashSet<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
    bounds: usize,
) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut path: Vec<(usize, usize)> = Vec::new();

    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((current, dist)) = queue.pop_front() {
        if current == end {
            return dist;
        }

        path.push(current);

        let (x, y) = current;
        let neighs = [
            (x.wrapping_sub(1), y),
            (x + 1, y),
            (x, y.wrapping_sub(1)),
            (x, y + 1),
        ];

        for next in neighs {
            if next.0 < bounds
                && next.1 < bounds
                && !obstacles.contains(&next)
                && !visited.contains(&next)
            {
                visited.insert(next);
                queue.push_back((next, dist + 1));
            }
        }
    }

    usize::MAX
}

fn part2(
    obstacles: Vec<(usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
    bounds: usize,
) -> (usize, usize) {
    let mut l = 1024;
    let mut r = obstacles.len();
    let mut m = 0;

    while l < r {
        m = (r + l + 1) / 2;

        let obstacles: HashSet<(usize, usize)> =
            obstacles.iter().take(m).map(|l| l.clone()).collect();

        let result = part1(obstacles, start, end, bounds);

        if result == usize::MAX {
            r = m - 1;
        } else {
            l = m;
        }
    }

    return obstacles[m];
}

fn main() {
    let input: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let tuples: Vec<(usize, usize)> = input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let lhs = parts[0].parse::<usize>().unwrap();
            let rhs = parts[1].parse::<usize>().unwrap();
            (lhs, rhs)
        })
        .collect();

    let target = 70;
    let size = target + 1;
    let bytes = 1024;
    let start = (0, 0);
    let end = (target, target);

    let mut score = vec![vec![usize::MAX / 2; size]; size];
    score[target][target] = 0;

    let obstacles: HashSet<(usize, usize)> = tuples.iter().take(bytes).map(|l| l.clone()).collect();

    let p1 = part1(obstacles, start, end, size);
    println!("Part 1: {}", p1);

    let p2 = part2(tuples, start, end, size);
    println!("Part 2: {},{}", p2.0, p2.1);
}
