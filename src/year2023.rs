use std::collections::{hash_map::RandomState, HashMap};
use std::collections::{HashSet, VecDeque};
use std::iter::repeat;
use std::ops::{Range, RangeInclusive};

pub fn get_day(day: &str) -> fn(&str) {
    match day {
        "1a" => p1a,
        "1b" => p1b,
        "2a" => p2a,
        "2b" => p2b,
        "3a" => p3a,
        "3b" => p3b,
        "4a" => p4a,
        "4b" => p4b,
        "5a" => p5a,
        "5b" => p5b,
        "6a" => p6a,
        "6b" => p6b,
        "7a" => p7a,
        "7b" => p7b,
        "8a" => p8a,
        "8b" => p8b,
        "9a" => p9a,
        "9b" => p9b,
        "10a" => p10a,
        "10b" => p10b,
        "11a" => p11a,
        "11b" => p11b,
        "12a" => p12a,
        "12b" => p12b,
        "13a" => p13a,
        "13b" => p13b,
        "14a" => p14a,
        "14b" => p14b,
        "15a" => p15a,
        "15b" => p15b,
        "16a" => p16a,
        "16b" => p16b,
        "17a" => p17a,
        "17b" => p17b,
        "18a" => p18a,
        "18b" => p18b,
        "19a" => p19a,
        "19b" => p19b,
        "20a" => p20a,
        "20b" => p20b,
        "21a" => p21a,
        "21b" => p21b,
        "22a" => p22a,
        "22b" => p22b,
        "23a" => p23a,
        "23b" => p23b,
        "24a" => p24a,
        "24b" => p24b,
        "25a" => p25a,
        "25b" => p25b,
        day => panic!("invalid solver: {day:?}"),
    }
}

fn p1a(input: &str) {
    let ans = input.lines().fold(0, |acc, l| {
        let mut iter = l.chars().filter_map(|x| x.to_digit(10));
        let first = iter.next().unwrap();
        let last = iter.next_back().unwrap_or(first);
        acc + first * 10 + last
    });
    println!("{ans}");
}

fn p1b(input: &str) {
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
        // check to see if the line starts/ends with a number or a word, otherwise shrink the slice and try again.
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

fn p2a(input: &str) {
    let limits =
        HashMap::<&str, usize, RandomState>::from_iter([("red", 12), ("green", 13), ("blue", 14)]);
    let ans = p2parse(input)
        // Remove any games that exceed the given limits
        .filter_map(|(game_id, rounds)| {
            { rounds.flatten() }
                .all(|(count, color)| count <= *limits.get(color).unwrap())
                .then_some(game_id)
        })
        .sum::<usize>();
    println!("{ans}");
}

fn p2b(input: &str) {
    let ans = p2parse(input)
        .map(|(_, rounds)| {
            // Find the minimum of each color required to make every round possible in this game.
            let (r, g, b) = rounds.flatten().fold((0, 0, 0), |mut acc, (count, color)| {
                let target = match color {
                    "red" => &mut acc.0,
                    "green" => &mut acc.1,
                    "blue" => &mut acc.2,
                    _ => unreachable!("hopefully"),
                };
                *target = count.max(*target);
                acc
            });
            r * g * b
        })
        .sum::<usize>();
    println!("{ans}");
}

fn p3a(input: &str) {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut num_start = None;
    let mut ans: usize = 0;
    for (y, row) in grid.iter().enumerate() {
        // chain an additional character to each row to avoid manual checking of num after the loop
        for (x, c) in row.iter().chain(Some(&b'.')).enumerate() {
            if c.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Some(x);
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
                // Look for symbols around the number to determine if it is a part.
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

fn p3b(input: &str) {
    let grid: Vec<&[u8]> = input.lines().map(str::as_bytes).collect();
    let mut num_start = None;
    // create a list of numbers with their bounding boxes to quickly find intersections
    let mut numbers = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        // chain an additional character to each row to avoid manual checking of num after the loop
        for (x, c) in row.iter().chain(Some(&b'.')).enumerate() {
            if c.is_ascii_digit() {
                if num_start.is_none() {
                    num_start = Some(x);
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

fn p4_matches_iter(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|l| {
        let (winners, given) = l.split_once(':').unwrap().1.split_once('|').unwrap();
        let [winners, given] = [winners, given].map(|list| {
            list.split(' ')
                .map(str::trim)
                .filter(|x| !x.is_empty())
                .collect::<HashSet<&str>>()
        });
        let intersection = winners.intersection(&given);
        intersection.count()
    })
}

fn p4a(input: &str) {
    let ans = p4_matches_iter(input)
        .map(|matches| {
            if matches > 0 {
                2usize.pow(u32::try_from(matches - 1).unwrap())
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("{ans}");
}

fn p4b(input: &str) {
    // Once a card is processed, its count is no longer needed.
    // Card counts only need to be stored if they are more than 1.
    let mut upcoming_card_counts = VecDeque::new();
    let ans = p4_matches_iter(input)
        .map(|matches| {
            let cards = upcoming_card_counts.pop_front().unwrap_or(1);
            for count in upcoming_card_counts.iter_mut().take(matches) {
                *count += cards;
            }
            if upcoming_card_counts.len() < matches {
                let diff = matches - upcoming_card_counts.len();
                upcoming_card_counts.extend((0..diff).map(|_| 1 + cards));
            }
            cards
        })
        .sum::<u32>();
    println!("{ans}");
}

fn p5a(input: &str) {
    let mut lines = input.lines();

    let seed_list_slice = lines.next().unwrap().split_once(": ").unwrap().1;
    let mut seeds = seed_list_slice
        .split(' ')
        .map(|seed| seed.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut mappings = Vec::<Vec<(Range<isize>, isize)>>::new();
    mappings.push(Vec::new());
    let mut last_map = mappings.last_mut().unwrap();

    lines.find(|l| l.contains(':'));
    for line in lines {
        if line.is_empty() {
        } else if line.contains(':') {
            mappings.push(Vec::new());
            last_map = mappings.last_mut().unwrap();
        } else {
            let mut iter = line.split(' ');
            let [dest, src, range] = [0; 3].map(|_| iter.next().unwrap().parse().unwrap());
            last_map.push((src..src + range, dest - src));
        }
    }

    for seed in &mut seeds {
        for map in &mappings {
            for (range, diff) in map {
                if range.contains(seed) {
                    *seed += diff;
                    break;
                }
            }
        }
    }

    println!("{}", seeds.iter().min().unwrap());
}

fn p5b(input: &str) {
    let mut lines = input.lines();

    let seed_list_slice = lines.next().unwrap().split_once(": ").unwrap().1;
    let mut iter = seed_list_slice
        .split(' ')
        .map(|seed| seed.parse::<isize>().unwrap());
    let mut seeds = Vec::new();
    let mut new_seeds = Vec::new();
    while let (Some(start), Some(range)) = (iter.next(), iter.next()) {
        if range > 0 {
            seeds.push(start..=start + range - 1);
        }
    }

    let mut mappings = Vec::<Vec<(RangeInclusive<isize>, isize)>>::new();
    mappings.push(Vec::new());
    let mut last_map = mappings.last_mut().unwrap();
    lines.find(|l| l.contains(':'));
    for line in lines {
        if line.is_empty() {
        } else if line.contains(':') {
            mappings.push(Vec::new());
            last_map = mappings.last_mut().unwrap();
        } else {
            let mut iter = line.split(' ');
            let [dest, src, range] = [0; 3].map(|_| iter.next().unwrap().parse().unwrap());
            last_map.push((src..=src + range - 1, dest - src));
        }
    }

    for map in &mappings {
        while let Some(mut seed) = seeds.pop() {
            for (src, diff) in map {
                // Some part of the seed range overlaps with the src range.
                if src.start() <= seed.end() && seed.start() <= src.end() {
                    // Break off the ends that aren't covered by src to be processed separately.
                    if seed.start() < src.start() {
                        seeds.push(*seed.start()..=*src.start() - 1);
                        seed = *src.start()..=*seed.end();
                    }
                    if src.end() < seed.end() {
                        seeds.push(*src.end() + 1..=*seed.end());
                        seed = *seed.start()..=*src.end();
                    }
                    seed = seed.start() + diff..=seed.end() + diff;
                    break;
                }
            }
            new_seeds.push(seed);
        }
        std::mem::swap(&mut seeds, &mut new_seeds);
    }

    println!("{}", seeds.iter().map(RangeInclusive::start).min().unwrap());
}

fn p6a(input: &str) {
    let mut lines = input.lines();
    let [times, distances] = [0; 2].map(|_| {
        let numbers_slice = lines.next().unwrap().split_once(':').unwrap().1;
        numbers_slice
            .split(' ')
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .map(|n| n.parse::<usize>().unwrap())
    });
    let ans = times
        .zip(distances)
        .map(|(time, distance)| (1..time - 1).filter(|t| (time - t) * t > distance).count())
        .product::<usize>();
    println!("{ans}");
}

fn p6b(input: &str) {
    let mut lines = input.lines();
    let [time, distance] = [0; 2].map(|_| {
        { lines.next().unwrap().bytes() }
            .filter(u8::is_ascii_digit)
            .map(|b| (b - b'0') as usize)
            .fold(0, |acc, n| acc * 10 + n)
    });
    let first_win = (1..time - 1).find(|t| (time - t) * t > distance).unwrap();
    println!("{}", time - (first_win - 1) * 2 - 1);
}

fn p7(input: &str, j_rule: bool) {
    let mut hands = input
        .lines()
        .map(|l| {
            let (hand, bid) = l.split_once(' ').unwrap();

            let hand_values = { hand.bytes() }
                .map(|b| match b {
                    b'A' => 14,
                    b'K' => 13,
                    b'Q' => 12,
                    b'J' => 1 + (u8::from(!j_rule) * 10),
                    b'T' => 10,
                    b'0'..=b'9' => b - b'0',
                    _ => panic!("bad char"),
                })
                .collect::<Vec<_>>();

            let mut kinds =
                { hand_values.iter() }.fold(HashMap::<u8, usize>::new(), |mut acc, &value| {
                    *acc.entry(value).or_default() += 1;
                    acc
                });

            if j_rule && kinds.len() > 1 {
                if let Some(j) = kinds.remove(&1) {
                    let target_count = kinds.iter_mut().max_by_key(|(_, count)| **count).unwrap();
                    *target_count.1 += j;
                }
            }

            let mut kinds = kinds.into_values().collect::<Vec<_>>();
            kinds.sort_unstable();
            kinds.reverse();
            let typ = match kinds.as_slice() {
                [5] => 7,
                [4, 1] => 6,
                [3, 2] => 5,
                [3, 1, 1] => 4,
                [2, 2, 1] => 3,
                [2, 1, 1, 1] => 2,
                [1, 1, 1, 1, 1] => 1,
                _ => unreachable!(),
            };

            (typ, hand, bid.parse().unwrap())
        })
        .collect::<Vec<_>>();

    hands.sort_unstable();

    let ans = { hands.iter().enumerate() }
        .map(|(i, (_, _, bid))| (i + 1) * bid)
        .sum::<usize>();

    println!("{ans}");
}

fn p7a(input: &str) {
    p7(input, false);
}

fn p7b(input: &str) {
    p7(input, true);
}

fn p8a(_input: &str) {}
fn p8b(_input: &str) {}

fn p9a(_input: &str) {}
fn p9b(_input: &str) {}

fn p10a(_input: &str) {}
fn p10b(_input: &str) {}

fn p11a(_input: &str) {}
fn p11b(_input: &str) {}

fn p12a(_input: &str) {}
fn p12b(_input: &str) {}

fn p13a(_input: &str) {}
fn p13b(_input: &str) {}

fn p14a(_input: &str) {}
fn p14b(_input: &str) {}

fn p15a(_input: &str) {}
fn p15b(_input: &str) {}

fn p16a(_input: &str) {}
fn p16b(_input: &str) {}

fn p17a(_input: &str) {}
fn p17b(_input: &str) {}

fn p18a(_input: &str) {}
fn p18b(_input: &str) {}

fn p19a(_input: &str) {}
fn p19b(_input: &str) {}

fn p20a(_input: &str) {}
fn p20b(_input: &str) {}

fn p21a(_input: &str) {}
fn p21b(_input: &str) {}

fn p22a(_input: &str) {}
fn p22b(_input: &str) {}

fn p23a(_input: &str) {}
fn p23b(_input: &str) {}

fn p24a(_input: &str) {}
fn p24b(_input: &str) {}

fn p25a(_input: &str) {}
fn p25b(_input: &str) {}
