use aoc2019::intcode;

use std::io::Read;

fn main() {
    let mut input: Vec<i32> = {
        let mut buf = String::new();
        std::io::stdin().lock().read_to_string(&mut buf).unwrap();
        buf.split(",")
            .into_iter()
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    };

    // adding the magic smoke...
    input[1] = 12;
    input[2] = 2;

    intcode::compute(&mut input);
    println!("{}", &input[0]);
}
