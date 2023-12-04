mod year2023;
use std::env::args;
use std::io::Read;

fn main() {
    let mut args = args();
    let year = args.nth(1).unwrap();
    let day = args.next().unwrap();
    let solver = match year.as_str() {
        "2023" => year2023::get_day(day.as_str()),
        year => panic!("invalid year {year:?}"),
    };

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    solver(input);
}
