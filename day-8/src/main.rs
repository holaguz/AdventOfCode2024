use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn is_bounded(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    return x >= 0 && y >= 0 && x < cols as isize && y < rows as isize;
}

fn part1(nodes: &HashMap<char, Vec<(usize, usize)>>, rows: usize, cols: usize) -> usize {
    let mut antinodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (char, positions) in nodes {
        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(i + 1) {
                let (dy, dx) = (
                    pos2.0 as isize - pos1.0 as isize,
                    pos2.1 as isize - pos1.1 as isize,
                );

                let a1 = (pos1.0 as isize - dy, pos1.1 as isize - dx);
                let a2 = (pos2.0 as isize + dy, pos2.1 as isize + dx);

                for (y, x) in [a1, a2] {
                    if !is_bounded(x, y, rows, cols) {
                        continue;
                    };

                    antinodes
                        .entry(*char)
                        .or_default()
                        .push((y as usize, x as usize));
                }
            }
        }
    }

    antinodes.values().flatten().collect::<HashSet<_>>().len()
}

fn part2(nodes: &HashMap<char, Vec<(usize, usize)>>, rows: usize, cols: usize) -> usize {
    let mut antinodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (char, positions) in nodes {
        for (i, &pos1) in positions.iter().enumerate() {
            for &pos2 in positions.iter().skip(i + 1) {
                let (dy, dx) = (
                    pos2.0 as isize - pos1.0 as isize,
                    pos2.1 as isize - pos1.1 as isize,
                );

                let mut ix = 0;
                let mut candidates: Vec<(isize, isize)> = Vec::new();

                loop {
                    let mut end = true;
                    let d1 = (pos1.0 as isize - dy * ix, pos1.1 as isize - dx * ix);
                    let d2 = (pos2.0 as isize + dy * ix, pos2.1 as isize + dx * ix);

                    if is_bounded(d1.1, d1.0, rows, cols) {
                        candidates.push(d1);
                        end = false;
                    }

                    if is_bounded(d2.1, d2.0, rows, cols) {
                        candidates.push(d2);
                        end = false;
                    }

                    if end {
                        break;
                    }

                    ix += 1;
                }

                for (y, x) in candidates {
                    antinodes
                        .entry(*char)
                        .or_default()
                        .push((y as usize, x as usize));
                }
            }
        }
    }

    antinodes.values().flatten().collect::<HashSet<_>>().len()
}

fn main() {
    let input: Vec<Vec<char>> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();

    let rows = input.len();
    let cols = input[0].len();

    let mut nodes: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'.' {
                continue;
            }

            nodes
                .entry(*c)
                .and_modify(|v| v.push((y, x)))
                .or_insert(vec![(y, x)]);
        }
    }

    let p1 = part1(&nodes, rows, cols);
    let p2 = part2(&nodes, rows, cols);

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
