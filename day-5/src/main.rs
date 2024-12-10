use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
};

fn is_good(rules: &HashMap<usize, Vec<usize>>, vec: &[usize]) -> Result<(), (usize, usize)> {
    let mut inserted: HashSet<usize> = HashSet::new();

    for (ix, value) in vec.iter().enumerate() {
        if let Some(values) = rules.get(value) {
            if let Some(prev) = inserted.iter().find(|prev| values.contains(prev)) {
                return Err((ix, *prev));
            }
        }

        inserted.insert(*value);
    }

    Ok(())
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines().map(|l| l.unwrap()).collect();

    let rules: Vec<(usize, usize)> = lines
        .iter()
        .filter_map(|l| {
            if !l.contains('|') {
                None
            } else {
                let nums: Vec<usize> = l
                    .split('|')
                    .map(|n| n.parse::<usize>().expect("Not a number"))
                    .collect();
                Some((nums[0], nums[1]))
            }
        })
        .collect();

    let updates: Vec<Vec<usize>> = lines
        .iter()
        .filter(|l| l.contains(','))
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<usize>().expect("not a number"))
                .collect::<Vec<usize>>()
        })
        .collect();

    let rules: HashMap<usize, Vec<usize>> =
        rules
            .into_iter()
            .fold(HashMap::new(), |mut map, (key, value)| {
                map.entry(key).or_default().push(value);
                map
            });

    let p1: usize = updates
        .iter()
        .map(|u| {
            if is_good(&rules, u).is_ok() {
                u[u.len() / 2]
            } else {
                0
            }
        })
        .sum();

    println!("Part 1: {}", p1);

    let p2: usize = updates
        .iter()
        .filter(|v| is_good(&rules, v).is_err())
        .map(|vec| {
            let mut vec = vec.clone();
            dbg!(&vec);
            loop {
                let (ix, bad_rule) = match is_good(&rules, vec.as_slice()) {
                    Ok(()) => break,
                    Err((ix, bad_rule)) => (ix, bad_rule),
                };

                let ix_bad_rule = vec
                    .iter()
                    .enumerate()
                    .find(|(_, v)| **v == bad_rule)
                    .unwrap()
                    .0;

                vec.swap(ix, ix_bad_rule);
            }

            vec[vec.len() / 2]
        })
        .sum();

    println!("Part 2: {}", p2);
}
