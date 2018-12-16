#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use bit_set::BitSet;

#[derive(Debug)]
struct Claim { x: u16, y: u16, w: u16, h: u16 }
lazy_static! {
    static ref line_re: Regex = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
}
impl Claim {
    fn parse(s: &str) -> Option<(u16, Claim)> {
        let caps = line_re.captures(s)?;
        // All of the unwraps are because the code should not be able to fail
        // if the regular expression happens. Failure indicates a logic error,
        // which should cause a panic.
        let mut ints =
            caps.iter()
                .skip(1)
                .map(|m| m .unwrap().as_str().parse::<u16>().unwrap());
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

    fn overlaps(&self, other: &Claim) -> bool {
        let x_overlaps =
            if self.x < other.x {
                other.x < self.x + self.w
            } else {
                self.x < other.x + other.w
            };
        let y_overlaps =
            if self.y < other.y {
                other.y < self.y + self.h
            } else {
                self.y < other.y + other.h
            };
        x_overlaps && y_overlaps
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let claims = BufReader::new(input)
        .lines()
        .map(|r| {
            let l = r.unwrap();
            Claim::parse(&l)
                .expect(&format!("Invalid line: {:?}", &l))
        })
        .collect::<Vec<(u16, Claim)>>();
    let mut found: Option<u16> = None;
    'outer: for (i, claim) in claims.iter() {
        for (j, claim2) in claims.iter() {
            if i != j && claim.overlaps(claim2) {
                continue 'outer;
            }
        }
        found = Some(*i);
        break;
    }
    println!("{:?}", found.unwrap());
}
