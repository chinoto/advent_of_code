pub fn get_day(day: &str) -> fn(String) {
    match day {
        "1a" => p1a,
        "1b" => p1b,
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
