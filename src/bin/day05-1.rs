use aoc2019::intcode;

use std::fs::File;
use std::io::Read;

fn main() {
    let mut input: Vec<i32> = {
        // On the day 2 binaries we read the input from stdin, but for day 5 the
        // "input" instructions will prompt for user input, so this time we read the
        // intcode source in from file instead. There may be a way to handle both
        // from stdin, but this works... While reading from stdin for the input here,
        // the read_line later only ever saw an empty string. Not sure why.
        let mut f = File::open(
            std::env::args()
                .nth(1)
                .expect("must supply intcode source as file path"),
        )
        .unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        buf.split(",")
            .into_iter()
            .filter_map(|s| s.trim().parse().ok())
            .collect()
    };

    intcode::compute(&mut input);
}
