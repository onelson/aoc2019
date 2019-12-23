//! # Day 2 1202 Program Alarm
//!
//! ## Part 1
//!
//! On the way to your gravity assist around the Moon, your ship computer beeps
//! angrily about a "1202 program alarm". On the radio, an Elf is already
//! explaining how to handle the situation: "Don't worry, that's perfectly norma--"
//!
//! The ship computer bursts into flames.
//!
//! You notify the Elves that the computer's magic smoke seems to have escaped.
//!
//! "That computer ran Intcode programs like the gravity assist program it was
//! working on; surely there are enough spare parts up there to build a new
//! Intcode computer!"
//!
//! An Intcode program is a list of integers separated by commas (like 1,0,0,3,99).
//! To run one, start by looking at the first integer (called position 0). Here,
//! you will find an opcode - either 1, 2, or 99. The opcode indicates what to do;
//! for example, 99 means that the program is finished and should immediately halt.
//! Encountering an unknown opcode means something went wrong.
//!
//! Opcode 1 adds together numbers read from two positions and stores the result
//! in a third position. The three integers immediately after the opcode tell you
//! these three positions - the first two indicate the positions from which you
//! should read the input values, and the third indicates the position at which
//! the output should be stored.
//!
//! For example, if your Intcode computer encounters 1,10,20,30, it should read
//! the values at positions 10 and 20, add those values, and then overwrite the
//! value at position 30 with their sum.
//!
//! Opcode 2 works exactly like opcode 1, except it multiplies the two inputs
//! instead of adding them. Again, the three integers after the opcode indicate
//! where the inputs and outputs are, not their values.
//!
//! Once you're done processing an opcode, move to the next one by stepping
//! forward 4 positions.
//!
//! For example, suppose you have the following program:
//!
//! ```text
//! 1,9,10,3,2,3,11,0,99,30,40,50
//! ```
//!
//! For the purposes of illustration, here is the same program split into
//! multiple lines:
//!
//! ```text
//! 1,9,10,3,
//! 2,3,11,0,
//! 99,
//! 30,40,50
//! ```
//!
//! The first four integers, `1,9,10,3`, are at positions `0`, `1`, `2`, and `3`.
//! Together, they represent the first opcode (`1`, addition), the positions of
//! the two inputs (`9` and `10`), and the position of the output (`3`).
//! To handle this opcode, you first need to get the values at the input positions:
//! position `9` contains `30`, and position `10` contains `40`. Add these numbers
//! together to get `70`. Then, store this value at the output position; here,
//! the output position (`3`) is at position `3`, so it overwrites itself.
//! Afterward, the program looks like this:
//!
//! ```text
//! 1,9,10,70,
//! 2,3,11,0,
//! 99,
//! 30,40,50
//! ```
//!
//! Step forward 4 positions to reach the next opcode, `2`.
//! This opcode works just like the previous, but it multiplies instead of adding.
//! The inputs are at positions `3` and `11`; these positions contain `70` and
//! `50` respectively. Multiplying these produces `3500`; this is stored at
//! position `0`:
//!
//! ```text
//! 3500,9,10,70,
//! 2,3,11,0,
//! 99,
//! 30,40,50
//! ```
//!
//! Stepping forward 4 more positions arrives at opcode `99`, halting the
//! program.
//!
//! Here are the initial and final states of a few more small programs:
//!
//! - `1,0,0,0,99` becomes `2,0,0,0,99` (1 + 1 = 2).
//! - `2,3,0,3,99` becomes `2,3,0,6,99` (3 * 2 = 6).
//! - `2,4,4,5,99,0` becomes `2,4,4,5,99,9801` (99 * 99 = 9801).
//! - `1,1,1,4,99,5,6,0,99` becomes `30,1,1,4,2,5,6,0,99`.
//!
//! Once you have a working computer, the first step is to restore the gravity
//! assist program (your puzzle input) to the "1202 program alarm" state it had
//! just before the last computer caught fire.
//! To do this, before running the program, replace position `1` with the value
//! `12` and replace position `2` with the value `2`. What value is left at
//! position `0` after the program halts?
//!
//! ## Part 2
//!
//! "Good, the new computer seems to be working correctly! Keep it nearby during
//! this mission - you'll probably use it again. Real Intcode computers support
//! many more features than your new one, but we'll let you know what they are
//! as you need them."
//!
//! "However, your current priority should be to complete your gravity assist
//! around the Moon. For this mission to succeed, we should settle on some
//! terminology for the parts you've already built."
//!
//! Intcode programs are given as a list of integers; these values are used as
//! the initial state for the computer's memory. When you run an Intcode program,
//! make sure to start by initializing memory to the program's values. A position
//! in memory is called an address (for example, the first value in memory is at
//! "address 0").
//!
//! Opcodes (like 1, 2, or 99) mark the beginning of an instruction. The values
//! used immediately after an opcode, if any, are called the instruction's
//! parameters. For example, in the instruction 1,2,3,4, 1 is the opcode; 2, 3,
//! and 4 are the parameters. The instruction 99 contains only an opcode and has
//! no parameters.
//!
//! The address of the current instruction is called the instruction pointer; it
//! starts at 0. After an instruction finishes, the instruction pointer increases
//! by the number of values in the instruction; until you add more instructions
//! to the computer, this is always 4 (1 opcode + 3 parameters) for the add and
//! multiply instructions. (The halt instruction would increase the instruction
//! pointer by 1, but it halts the program instead.)
//!
//! "With terminology out of the way, we're ready to proceed. To complete the
//! gravity assist, you need to determine what pair of inputs produces the
//! output 19690720."
//!
//! The inputs should still be provided to the program by replacing the values
//! at addresses 1 and 2, just like before. In this program, the value placed in
//! address 1 is called the noun, and the value placed in address 2 is called
//! the verb. Each of the two input values will be between 0 and 99, inclusive.
//!
//! Once the program has halted, its output is available at address 0, also just
//! like before. Each time you try a pair of inputs, make sure you first reset
//! the computer's memory to the values in the program (your puzzle input) - in
//! other words, don't reuse memory from a previous attempt.
//!
//! Find the input noun and verb that cause the program to produce the output
//! 19690720. What is 100 * noun + verb? (For example, if noun=12 and verb=2,
//! the answer would be 1202.)
//!
//! # Day 5: Sunny with a Chance of Asteroids
//!
//! ## Part 1
//!
//! You're starting to sweat as the ship makes its way toward Mercury. The Elves
//! suggest that you get the air conditioner working by upgrading your ship
//! computer to support the Thermal Environment Supervision Terminal.
//!
//! The Thermal Environment Supervision Terminal (TEST) starts by running a
//! diagnostic program (your puzzle input). The TEST diagnostic program will run
//! on your existing Intcode computer after a few modifications:
//!
//! First, you'll need to add **two new instructions**:
//!
//! - Opcode `3` takes a single integer as **input** and saves it to the position
//!   given by its only parameter. For example, the instruction `3,50` would take
//!   an input value and store it at address `50`.
//! - Opcode `4` **outputs** the value of its only parameter. For example, the
//!   instruction `4,50` would output the value at address `50`.
//!
//! Programs that use these instructions will come with documentation that
//! explains what should be connected to the input and output.
//! The program `3,0,4,0,99` outputs whatever it gets as input, then halts.
//!
//! Second, you'll need to add support for parameter modes:
//!
//! Each parameter of an instruction is handled based on its parameter mode.
//! Right now, your ship computer already understands parameter mode 0, position
//! mode, which causes the parameter to be interpreted as a position - if the
//! parameter is 50, its value is the value stored at address 50 in memory.
//! Until now, all parameters have been in position mode.
//!
//! Now, your ship computer will also need to handle parameters in mode 1,
//! immediate mode. In immediate mode, a parameter is interpreted as a value - if
//! the parameter is 50, its value is simply 50.
//!
//! Parameter modes are stored in the same value as the instruction's opcode.
//! The opcode is a two-digit number based only on the ones and tens digit of the
//! value, that is, the opcode is the rightmost two digits of the first value in
//! an instruction. Parameter modes are single digits, one per parameter, read
//! right-to-left from the opcode: the first parameter's mode is in the hundreds
//! digit, the second parameter's mode is in the thousands digit, the third
//! parameter's mode is in the ten-thousands digit, and so on.
//! Any missing modes are 0.
//!
//! For example, consider the program `1002,4,3,4,33`.
//!
//! The first instruction, `1002,4,3,4`, is a multiply instruction - the rightmost
//! two digits of the first value, 02, indicate opcode 2, multiplication.
//! Then, going right to left, the parameter modes are 0 (hundreds digit),
//! 1 (thousands digit), and 0 (ten-thousands digit, not present and therefore
//! zero):
//!
//! ```text
//! ABCDE
//! 1002
//!
//! DE - two-digit opcode,      02 == opcode 2
//! C - mode of 1st parameter,  0 == position mode
//! B - mode of 2nd parameter,  1 == immediate mode
//! A - mode of 3rd parameter,  0 == position mode,
//!                                  omitted due to being a leading zero
//! ```
//!
//! This instruction multiplies its first two parameters.
//! The first parameter, 4 in position mode, works like it did before - its value
//! is the value stored at address 4 (33). The second parameter, 3 in immediate
//! mode, simply has value 3. The result of this operation, 33 * 3 = 99, is written
//! according to the third parameter, 4 in position mode, which also works like
//! it did before - 99 is written to address 4.
//!
//! Parameters that an instruction writes to will never be in immediate mode.
//!
//! Finally, some notes:
//!
//! - It is important to remember that the instruction pointer should increase by
//!   the number of values in the instruction after the instruction finishes.
//!   Because of the new instructions, this amount is no longer always `4`.
//! - Integers can be negative: `1101,100,-1,4,0` is a valid program (find
//!   `100 + -1`, store the result in position `4`).
//!
//! The TEST diagnostic program will start by requesting from the user the ID of
//! the system to test by running an input instruction - provide it 1, the ID for
//! the ship's air conditioner unit.
//!
//! It will then perform a series of diagnostic tests confirming that various
//! parts of the Intcode computer, like parameter modes, function correctly.
//! For each test, it will run an output instruction indicating how far the result
//! of the test was from the expected value, where 0 means the test was successful.
//! Non-zero outputs mean that a function is not working correctly; check the
//! instructions that were run before the output instruction to see which one
//! failed.
//!
//! Finally, the program will output a diagnostic code and immediately halt.
//! This final output isn't an error; an output followed immediately by a halt
//! means the program finished. If all outputs were zero except the diagnostic
//! code, the diagnostic program ran successfully.
//!
//! After providing 1 to the only input instruction and passing all the tests,
//! what diagnostic code does the program produce?

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct OpHeader {
    mode1: usize,
    mode2: usize,
    mode3: usize,
    opcode: usize,
}

impl FromStr for OpHeader {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // initial string should not be larger than 5 or smaller than 1 chars.
        if s.len() > 5 || s.len() < 1 {
            return Err(());
        }

        let padded = format!("{:0>5}", s.chars().take(5).collect::<String>());
        let (modes, opcode) = padded.split_at(3);

        let modes: Vec<u32> = modes.chars().filter_map(|c| c.to_digit(10)).collect();
        let opcode: usize = opcode.parse().map_err(|_| ())?;

        Ok(OpHeader {
            mode1: modes[0] as usize,
            mode2: modes[1] as usize,
            mode3: modes[2] as usize,
            opcode,
        })
    }
}

impl TryFrom<i32> for OpHeader {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        value.to_string().parse()
    }
}

#[derive(Debug)]
enum Param {
    Immediate(i32),
    Position(usize),
}

#[derive(Debug)]
enum Op {
    Add { a: Param, b: Param, out: usize },
    Multiply { a: Param, b: Param, out: usize },
    Input { value: Param },
    Output { value: Param },
    Halt,
    Unknown,
}

/// Builds an `Op` from `data` by reading up to 4 items from a given offset.
fn read_instruction(offset: usize, data: &[i32]) -> Op {
    // FIXME: check opcode to decide on param types
    // FIXME: add support for Input/Output
    let header: Option<OpHeader> = data.get(offset).and_then(|x| OpHeader::try_from(*x).ok());
    match (
        header.as_ref(),
        data.get(offset + 1).map(|x| *x as usize),
        data.get(offset + 2).map(|x| *x as usize),
        data.get(offset + 3).map(|x| *x as usize),
    ) {
        (Some(OpHeader { opcode: 1, .. }), Some(a), Some(b), Some(out)) => Op::Add {
            a: Param::Position(a),
            b: Param::Position(b),
            out,
        },
        (Some(OpHeader { opcode: 2, .. }), Some(a), Some(b), Some(out)) => Op::Multiply {
            a: Param::Position(a),
            b: Param::Position(b),
            out,
        },
        (Some(OpHeader { opcode: 99, .. }), _, _, _) => Op::Halt,
        _ => Op::Unknown,
    }
}

/// Run an intcode program.
pub fn compute(data: &mut [i32]) {
    let mut i = 0;
    loop {
        match read_instruction(i, &data) {
            Op::Add { a, b, out } => {
                let a = match a {
                    Param::Position(idx) => data[idx],
                    Param::Immediate(val) => val,
                };
                let b = match b {
                    Param::Position(idx) => data[idx],
                    Param::Immediate(val) => val,
                };
                data[out] = a + b;
            }
            Op::Multiply { a, b, out } => {
                let a = match a {
                    Param::Position(idx) => data[idx],
                    Param::Immediate(val) => val,
                };
                let b = match b {
                    Param::Position(idx) => data[idx],
                    Param::Immediate(val) => val,
                };
                data[out] = a * b;
            }
            Op::Halt => break,
            _ => unreachable!(),
        }
        i += 4; // FIXME: increment based on Op length
    }
}

/// Attempt to identify the noun and verb (injected header values) which will
/// yield a certain target from a source intcode program by way of permutations.
pub fn solve(target: i32, data: &[i32]) -> Result<(i32, i32), ()> {
    for (noun, verb) in (0..=99).flat_map(|i| (0..=99).map(move |j| (i, j))) {
        let mut input = data.to_vec();
        input[1] = noun;
        input[2] = verb;
        compute(&mut input);
        if input[0] == target {
            return Ok((noun, verb));
        }
    }
    Err(())
}

#[cfg(test)]
mod day02_1_tests {
    use super::compute;

    #[test]
    fn test_example_1() {
        let mut input = vec![1, 0, 0, 0, 99];
        compute(&mut input);
        assert_eq!(&input, &[2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_example_2() {
        let mut input = vec![2, 3, 0, 3, 99];
        compute(&mut input);
        assert_eq!(&input, &[2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_example_3() {
        let mut input = vec![2, 4, 4, 5, 99, 0];
        compute(&mut input);
        assert_eq!(&input, &[2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_example_4() {
        let mut input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        compute(&mut input);
        assert_eq!(&input, &[30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}

#[cfg(test)]
mod day05_1_tests {
    use super::{compute, OpHeader};

    #[test]
    fn test_header_pad() {
        // when the header is "short" it should be padded on the left with zeros
        assert_eq!(
            "102".parse::<OpHeader>().unwrap(),
            OpHeader {
                mode1: 0,
                mode2: 0,
                mode3: 1,
                opcode: 2
            }
        );
    }

    #[test]
    fn test_header_parse_two_digit_opcode() {
        assert_eq!(
            "1022".parse::<OpHeader>().unwrap(),
            OpHeader {
                mode1: 0,
                mode2: 1,
                mode3: 0,
                opcode: 22
            }
        );
    }

    #[test]
    fn test_example_1() {
        let mut input = vec![1002, 4, 3, 4, 33];
        compute(&mut input);
        assert_eq!(&input, &[1002, 4, 3, 4, 99]);
    }
}
