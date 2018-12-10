use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::BTreeSet;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut seen = BTreeSet::new();
    let deltas: Vec<i32> =
    BufReader::new(input).lines().map(|line_res| {
        let line = line_res.unwrap();
        let (op, arg_str) = line.split_at(1);
        let arg = arg_str.parse::<i32>().unwrap();
        match op {
            "-" => -arg,
            "+" => arg,
            other => panic!(format!("invalid op: {}", other))
        }
    }).collect();
        let mut total = 0;
    let mut i = 0;
    for d in deltas.iter().cycle() {
        if seen.contains(&total) {
            break;
        } else {
            seen.insert(total);
        }
        i += 1;
        total += d;
    }
    println!("after {} ({} deltas), found {}", i, deltas.len(), total);
}
