use std::io;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut total = 0;
    for line_res in BufReader::new(input).lines() {
        let line = line_res.unwrap();
        let (op, arg_str) = line.split_at(1);
        let arg = arg_str.parse::<i32>().unwrap();
        match op {
            "-" => total = total - arg,
            "+" => total = total + arg,
            other => panic!(format!("invalid op: {}", other))
        };
    }
    println!("{}", total);
}
