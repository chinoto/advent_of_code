use std::collections::{hash_map::RandomState, HashMap};
use std::iter::repeat;

pub fn get_day(day: &str) -> fn(String) {
    match day {
        "1a" => p1a,
        "1b" => p1b,
        "2a" => p2a,
        "2b" => p2b,
        "3a" => p3a,
        "3b" => p3b,
        day => panic!("invalid or unimplemented solver: {day:?}"),
    }
}

fn p1a(input: String) {
    let ans = input.lines().fold(0, |acc, l| {
        let mut iter = l.chars();
        let first = iter.find_map(|x| x.to_digit(10)).unwrap();
        let last = iter.rev().find_map(|x| x.to_digit(10)).unwrap_or(first);
        acc + first * 10 + last
    });
    println!("{ans}");
}

fn p1b(input: String) {
    let map: &[(_, &[u8])] = &[
        (1, b"one"),
        (2, b"two"),
        (3, b"three"),
        (4, b"four"),
        (5, b"five"),
        (6, b"six"),
        (7, b"seven"),
        (8, b"eight"),
        (9, b"nine"),
    ];

    let ans = input.lines().fold(0usize, |acc, l| {
        let mut slice = l.as_bytes();
        let first = loop {
            if (b'1'..=b'9').contains(&slice[0]) {
                break slice[0] - b'0';
            }
            if let Some(n) = { map.iter() }.find_map(|&(n, w)| slice.starts_with(w).then_some(n)) {
                break n;
            }
            slice = &slice[1..];
        };

        let mut slice = l.as_bytes();
        let last = loop {
            let last = slice.last().unwrap();
            if (b'1'..=b'9').contains(last) {
                break last - b'0';
            }
            if let Some(n) = { map.iter() }.find_map(|&(n, w)| slice.ends_with(w).then_some(n)) {
                break n;
            }
            slice = &slice[..slice.len() - 1];
        };

        acc + (first * 10 + last) as usize
    });
    println!("{ans}");
}

fn p2parse(
    input: &str,
) -> impl Iterator<
    Item = (
        usize,
        impl Iterator<Item = impl Iterator<Item = (usize, &str)>>,
    ),
> {
    input.lines().map(|l| {
        let (game, rounds) = l.split_once(':').unwrap();
        (
            game.split_once(' ').unwrap().1.parse::<usize>().unwrap(),
            rounds.split(';').map(|round| {
                round.split(',').map(|count_color| {
                    let (count, color) = count_color.trim().split_once(' ').unwrap();
                    (count.parse().unwrap(), color)
                })
            }),
        )
    })
}

fn p2a(input: String) {
    let limits =
        HashMap::<&str, usize, RandomState>::from_iter([("red", 12), ("green", 13), ("blue", 14)]);
    let ans = p2parse(&input)
        .filter_map(|(game_id, rounds)| {
            for round in rounds {
                for (count, color) in round {
                    if *limits.get(color).unwrap() < count {
                        return None;
                    }
                }
            }
            Some(game_id)
        })
        .sum::<usize>();
    println!("{ans}");
}

fn p2b(input: String) {
    let ans = p2parse(&input)
        .map(|(_, rounds)| {
            let (r, g, b) = rounds.fold((0, 0, 0), |mut acc, round| {
                for (count, color) in round {
                    let target = match color {
                        "red" => &mut acc.0,
                        "green" => &mut acc.1,
                        "blue" => &mut acc.2,
                        _ => unreachable!("hopefully"),
                    };
                    *target = count.max(*target);
                }
                acc
            });
            r * g * b
        })
        .sum::<usize>();
    println!("{ans}");
}

fn p3a(input: String) {
    let grid: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let mut num_start = None;
    let mut ans: usize = 0;
    for (y, row) in grid.iter().enumerate() {
        // chain an additional character to each row to avoid manual checking of num after the loop
        for (x, c) in row.iter().chain(Some(&b'.')).enumerate() {
            if c.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Some(x)
                }
            } else if let Some(start) = num_start {
                let above_row_iter = { y.checked_sub(1) }
                    .map(|above| (start.saturating_sub(1)..=x).zip(repeat(above)))
                    .into_iter()
                    .flatten();
                let mut border_iter = above_row_iter
                    .chain(start.checked_sub(1).map(|left| (left, y)))
                    .chain(Some((x, y)))
                    .chain((start.saturating_sub(1)..=x).zip(repeat(y + 1)))
                    .filter_map(|(x, y)| grid.get(y)?.get(x));
                if border_iter.any(|&c| !c.is_ascii_digit() && c != b'.') {
                    ans += row[start..x]
                        .iter()
                        .fold(0, |acc, b| acc * 10 + ((b - b'0') as usize));
                }
                num_start = None;
            }
        }
    }
    println!("{ans}");
}

fn p3b(input: String) {
    let grid: Vec<&[u8]> = input.lines().map(|x| x.as_bytes()).collect();
    let mut num_start = None;
    // create a list of numbers with their bounding boxes to quickly find intersections
    let mut numbers = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        // chain an additional character to each row to avoid manual checking of num after the loop
        for (x, c) in row.iter().chain(Some(&b'.')).enumerate() {
            if c.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Some(x)
                }
            } else if let Some(start) = num_start {
                let number = row[start..x]
                    .iter()
                    .fold(0, |acc, b| acc * 10 + ((b - b'0') as usize));
                numbers.push((number, start..=x - 1, y));
                num_start = None;
            }
        }
    }

    let mut ans: usize = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != b'*' {
                continue;
            }
            let gear_x = x.saturating_sub(1)..=x + 1;
            let gear_y = y.saturating_sub(1)..=y + 1;
            let mut iter = numbers.iter().filter_map(|(num, num_x, num_y)| {
                (gear_y.contains(num_y)
                    && gear_x.start() <= num_x.end()
                    && num_x.start() <= gear_x.end())
                .then_some(num)
            });
            if let (Some(a), Some(b)) = (iter.next(), iter.next()) {
                ans += a * b;
            }
            assert_eq!(iter.next(), None);
        }
    }
    println!("{ans}");
}
