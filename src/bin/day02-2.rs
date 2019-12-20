use aoc2019::intcode;

use std::io::Read;

fn main() {
    let input: Vec<i32> = {
        let mut buf = String::new();
        std::io::stdin().lock().read_to_string(&mut buf).unwrap();
        buf.split(",")
            .into_iter()
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    };

    match intcode::solve(19690720, &input) {
        Ok((noun, verb)) => {
            println!("noun={} verb={}", noun, verb);
            println!("{}", 100 * noun + verb);
        }
        Err(_) => eprintln!("target not found"),
    }
}
