//! # Day 4: Secure Container
//!
//! ## Part 1
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a
//! password. The Elves had written the password on a sticky note, but someone
//! threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//! - It is a six-digit number.
//! - The value is within the range given in your puzzle input.
//! - Two adjacent digits are the same (like `22` in `1`**`22`**`345`).
//! - Going from left to right, the digits **never decrease**; they only ever
//!   increase or stay the same (like `111123` or `135679`).
//!
//! Other than the range rule, the following are true:
//!
//! - `111111` meets these criteria (double 11, never decreases).
//! - `2234`**`50`** does not meet these criteria (decreasing pair of digits `50`).
//! - `123789` does not meet these criteria (no double).
//!
//! How many different passwords within the range given in your puzzle input
//! meet these criteria?

/// Checks to see if a number conforms to the password policy.
pub fn is_valid(n: u32) -> bool {
    let digits = to_digits(n);
    if digits.len() != 6 {
        return false;
    }

    let mut sorted = digits.clone();
    sorted.sort();
    if sorted != digits {
        return false;
    }

    let mut prev = -1;
    for i in &digits {
        if prev == *i as i32 {
            return true;
        }
        prev = *i as i32;
    }

    return false;
}

/// Convert a non-negative number into a series of individual digits.
fn to_digits(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect()
}

#[cfg(test)]
mod day04_1_tests {
    use super::*;

    #[test]
    fn test_to_digits() {
        assert_eq!(&to_digits(123), &[1, 2, 3]);
        assert_eq!(&to_digits(141412), &[1, 4, 1, 4, 1, 2]);
    }

    #[test]
    fn test_111111_is_valid() {
        assert!(is_valid(111111));
    }

    #[test]
    fn test_223450_is_not_valid() {
        assert!(!is_valid(223450));
    }

    #[test]
    fn test_123789_is_not_valid() {
        assert!(!is_valid(123789));
    }
}
