use std::io::{stdin, BufRead};

fn walk(
    mat: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    rows: usize,
    cols: usize,
    dir: (isize, isize),
) -> usize {
    let next = match mat[y][x] {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        'S' => return 1,
        _ => panic!(""),
    };

    let mut ans = 0;
    let nx = x as isize + dir.0;
    let ny = y as isize + dir.1;

    if nx < 0 || nx >= cols as isize || ny < 0 || ny >= rows as isize {
        return 0;
    }

    if mat[ny as usize][nx as usize] == next {
        ans += walk(mat, nx as usize, ny as usize, rows, cols, dir);
    }

    ans
}

fn part2(mat: &Vec<Vec<char>>, x: usize, y: usize, rows: usize, cols: usize) -> usize {
    if x == 0 || y == 0 || x + 1 >= cols || y + 1 >= rows {
        return 0;
    }

    let vec: Vec<_> = [(-1, -1), (1, -1), (-1, 1), (1, 1)]
        .iter()
        .map(|(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            mat[ny as usize][nx as usize]
        })
        .collect();

    match vec.as_slice() {
        &['M', 'S', 'M', 'S']
        | &['M', 'M', 'S', 'S']
        | &['S', 'S', 'M', 'M']
        | &['S', 'M', 'S', 'M'] => 1,
        _ => 0,
    }
}

fn main() {
    let input: Vec<_> = stdin().lock().lines().map(|l| l.unwrap()).collect();
    let mat: Vec<Vec<char>> = input.iter().map(|l| l.chars().collect()).collect();

    let rows = mat.len();
    let cols = mat[0].len();

    let mut p1 = 0;
    let mut p2 = 0;
    for (y, row) in mat.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            match *char {
                'X' => {
                    for dy in [-1, 0, 1] {
                        for dx in [-1, 0, 1] {
                            if dx == 0 && dy == 0 {
                                continue;
                            };

                            p1 += walk(&mat, x, y, rows, cols, (dy, dx));
                        }
                    }
                }
                'A' => p2 += part2(&mat, x, y, rows, cols),
                _ => continue,
            }
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
