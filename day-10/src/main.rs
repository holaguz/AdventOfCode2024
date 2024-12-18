use std::io::{stdin, BufRead};

fn walk(
    mat: &Vec<Vec<usize>>,
    vis: &mut Vec<Vec<bool>>,
    x: usize,
    y: usize,
    rows: usize,
    cols: usize,
    all_paths: bool,
) -> usize {
    let cur = mat[y][x];
    let next = cur + 1;

    if !all_paths {
        if vis[y][x] {
            return 0;
        } else {
            vis[y][x] = true;
        }
    }

    if cur == 9 {
        return 1;
    };

    let mut ans = 0;

    for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if ny < 0
            || nx < 0
            || ny >= rows as isize
            || nx >= cols as isize
            || mat[ny as usize][nx as usize] != next
        {
            continue;
        }

        ans += walk(mat, vis, nx as usize, ny as usize, rows, cols, all_paths);
    }

    ans
}

fn main() {
    let input: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mat: Vec<Vec<usize>> = input
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let rows = mat.len();
    let cols = mat[0].len();

    let mut start_pos: Vec<(usize, usize)> = Vec::new();
    for (y, row) in mat.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 0 {
                start_pos.push((x, y));
            }
        }
    }

    dbg!(start_pos.len());

    let scores: Vec<_> = start_pos
        .iter()
        .map(|(x, y)| {
            let mut vis = vec![vec![false; cols]; rows];
            walk(&mat, &mut vis, *x, *y, rows, cols, false)
        })
        .collect();

    println!("Part 1: {}", scores.iter().sum::<usize>());

    let scores: Vec<_> = start_pos
        .iter()
        .map(|(x, y)| {
            let mut vis = vec![vec![false; cols]; rows];
            walk(&mat, &mut vis, *x, *y, rows, cols, true)
        })
        .collect();

    println!("Part 2: {}", scores.iter().sum::<usize>());
}
