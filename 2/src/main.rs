use std::io;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Copy, Debug, Clone)]
struct Categories {
    two_char: bool,
    three_char: bool
}

fn categorize(the_string: &str) -> Categories {
    let mut counts = [0; 26];
    for c in the_string.chars() {
        let base =
            if c >= 'a' && c <= 'z' {
                'a'
            } else if c >= 'A' && c <= 'Z' {
                'A'
            } else {
                continue
            };
        counts[c as usize - base as usize] += 1;
    }
    Categories { two_char: counts.contains(&2), three_char: counts.contains(&3) }
}

fn is_match(s1: &str, s2: &str) -> Option<Vec<char>> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut flipped = false;
    let mut v = Vec::new();
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 == c2 {
            v.push(c1)
        } else if flipped {
            return None
        } else {
            flipped = true;
        }
    }
    if flipped {
        Some(v)
    } else {
        None
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut twos = 0;
    let mut threes = 0;
    let words: Vec<String> =
        BufReader::new(input).lines().map(|l|l.unwrap()).collect();

    for line in words.iter() {
        let cat = categorize(&line);
        if cat.two_char {
            twos += 1;
        }
        if cat.three_char {
            threes += 1;
        }
    }

    'outer: for w1 in words.iter() {
        for w2 in words.iter() {
            match is_match(&w1, &w2) {
                Some(cs) => {
                    println!("{:?}", cs.iter().map(|c| *c).collect::<String>());
                    break 'outer
                },
                None => ()
            }
        }
    }

    println!("{}", twos * threes);
}
