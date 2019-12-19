use aoc2019::crossed_wires;

use std::io::BufRead;

fn main() {
    let inputs: Vec<String> = {
        std::io::stdin()
            .lock()
            .lines()
            .filter_map(|line| line.ok())
            .collect()
    };

    let a = crossed_wires::build_steps(&inputs[0]);
    let b = crossed_wires::build_steps(&inputs[1]);
    println!(
        "{}",
        crossed_wires::calc_fewest_steps(&a, &b).expect("Wires don't cross.")
    );
}
