#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    x: i32,
    y: i32,
    w: i32,
    h: i32
}
lazy_static! {
    static ref line_re: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}
impl Claim {
    fn parse(s: &str) -> Option<(i32, Claim)> {
        let caps = line_re.captures(s)?;
        // All of the unwraps are because the code should not be able to fail
        // if the regular expression happens. Failure indicates a logic error,
        // which should cause a panic.
        let mut ints =
            caps.iter()
                .skip(1)
                .map(|m| m .unwrap().as_str().parse::<i32>().unwrap());
        let idx = ints.next().unwrap();
        let claim =
            Claim {
                x: ints.next().unwrap(),
                y: ints.next().unwrap(),
                w: ints.next().unwrap(),
                h: ints.next().unwrap()
            };
        assert!(ints.next() == None, "iterator exhausted");
        Some((idx, claim))
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    BufReader::new(input)
        .lines()
        .map(|l| Claim::parse(&l.unwrap()))
        .for_each(|x|println!("{:#?}", x));
}
