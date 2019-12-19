use aoc2019::secure_container;

fn main() {
    let start: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap();
    let end: u32 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap();

    let count = (start..=end)
        .into_iter()
        .filter(|&x| secure_container::is_more_valid(x))
        .count();
    println!("{}", count);
}
