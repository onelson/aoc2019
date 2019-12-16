//! --- Day 3: Crossed Wires ---
//!
//! --- Part One ---
//! The gravity assist was successful, and you're well on your way to the Venus
//! refuelling station. During the rush back on Earth, the fuel management system
//! wasn't completely installed, so that's next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, two wires
//! are connected to a central port and extend outward on a grid. You trace the
//! path each wire takes as it leaves the central port, one wire per line of
//! text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix
//! the circuit, you need to find the intersection point closest to the central
//! port. Because the wires are on a grid, use the Manhattan distance for this
//! measurement. While the wires do technically cross right at the central port
//! where they both start, this point does not count, nor does a wire count as
//! crossing with itself.
//!
//! For example, if the first wire's path is R8,U5,L5,D3, then starting from the
//! central port (o), it goes right 8, up 5, left 5, and finally down 3:
//!
//! ```text
//! ...........
//! ...........
//! ...........
//! ....+----+.
//! ....|....|.
//! ....|....|.
//! ....|....|.
//! .........|.
//! .o-------+.
//! ...........
//! ```
//!
//! Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6, down
//! 4, and left 4:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! These wires cross at two locations (marked X), but the lower-left one is
//! closer to the central port: its distance is 3 + 3 = 6.
//!
//! Here are a few more examples:
//!
//! - R75,D30,R83,U83,L12,D49,R71,U7,L72
//!   U62,R66,U55,R34,D71,R55,D58,R83 = distance 159
//! - R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//!   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = distance 135
//!
//! What is the Manhattan distance from the central port to the closest intersection?
//!
//! --- Part Two ---
//! It turns out that this circuit is very timing-sensitive; you actually need
//! to minimize the signal delay.
//!
//! To do this, calculate the number of steps each wire takes to reach each
//! intersection; choose the intersection where the sum of both wires' steps is
//! lowest.
//! If a wire visits a position on the grid multiple times, use the steps value
//! from the first time it visits that position when calculating the total value
//! of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the
//! wire has entered to get to that location, including the intersection being
//! considered. Again consider the example from above:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! In the above example, the intersection closest to the central port is reached
//! after 8+5+5+2 = 20 steps by the first wire and 7+6+4+3 = 20 steps by the
//! second wire for a total of 20+20 = 40 steps.
//!
//! However, the top-right intersection is better: the first wire takes only
//! 8+5+2 = 15 and the second wire takes only 7+6+2 = 15, a total of 15+15 = 30 steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//! - R75,D30,R83,U83,L12,D49,R71,U7,L72
//!   U62,R66,U55,R34,D71,R55,D58,R83 = 610 steps
//! - R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//!   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7 = 410 steps
//!
//! What is the fewest combined steps the wires must take to reach an intersection?

use std::collections::HashSet;
use std::iter::IntoIterator;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Left(i32),
    Right(i32),
    Up(i32),
    Down(i32),
}

type Point = (i32, i32);

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();

        let (head, tail) = chars.split_at(1);
        let distance = tail
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .expect("dist parse");
        match head {
            ['L'] => Ok(Step::Left(distance)),
            ['R'] => Ok(Step::Right(distance)),
            ['U'] => Ok(Step::Up(distance)),
            ['D'] => Ok(Step::Down(distance)),
            _ => Err(()),
        }
    }
}

pub fn calc_distance(a: &[Step], b: &[Step]) -> i32 {
    let a: HashSet<_> = plot_course(a)
        .into_iter()
        .flat_map(|xs| xs.into_iter())
        .collect();
    let b: HashSet<_> = plot_course(b)
        .into_iter()
        .flat_map(|xs| xs.into_iter())
        .collect();

    a.intersection(&b)
        .into_iter()
        .map(|point| point.0.abs() + point.1.abs())
        .min()
        .expect("min distance")
}

#[derive(Clone, Default)]
struct PathWalk {
    step_count: usize,
    points: Vec<Vec<Point>>,
    visited: HashSet<Point>,
}

impl PathWalk {
    pub fn new(points: Vec<Vec<Point>>) -> Self {
        Self {
            points,
            ..Default::default()
        }
    }

    pub fn crosses(&self, other: &PathWalk) -> bool {
        other.visited.intersection(&self.visited).count() > 0
    }

    // Creates a new PathWalk which moves the next pending step into the
    // visited set.
    pub fn next(&self) -> Option<PathWalk> {
        self.points.split_first().map(|(head, tail)| {
            let mut visited = self.visited.clone();
            visited.extend(head);
            let step_count = self.step_count + head.len();
            Self {
                points: tail.to_vec(),
                step_count,
                visited,
            }
        })
    }
}

fn walk(a: PathWalk, b: PathWalk) -> Option<usize> {
    if a.crosses(&b) {
        return Some(a.step_count + b.step_count);
    }
    match (a.next(), b.next()) {
        (None, None) => None,
        (Some(next_a), None) => walk(next_a, b),
        (None, Some(next_b)) => walk(a, next_b),
        (Some(next_a), Some(next_b)) => {
            let walked_a = walk(a, next_b);
            let walked_b = walk(b, next_a);
            [walked_a, walked_b].into_iter().filter_map(|x| *x).min()
        }
    }
}

pub fn calc_fewest_steps(a: &[Step], b: &[Step]) -> Option<usize> {
    let walker_a = PathWalk::new(plot_course(a));
    let walker_b = PathWalk::new(plot_course(b));
    walk(walker_a, walker_b)
}

fn plot_course(directions: &[Step]) -> Vec<Vec<Point>> {
    let mut buf: Vec<Vec<Point>> = vec![];
    for step in directions {
        // Eh, this is meant to be the last item in the last sub-vec, or `(0, 0)`
        // if we're just starting from an empty `buf`.
        let &(x, y) = buf
            .iter()
            .last()
            .and_then(|x| x.iter().last())
            .unwrap_or(&(0, 0));

        match step {
            &Step::Left(dist) => buf.push((1..=dist as usize).map(|i| (x - i as i32, y)).collect()),
            &Step::Right(dist) => {
                buf.push((1..=dist as usize).map(|i| (x + i as i32, y)).collect())
            }
            &Step::Up(dist) => buf.push((1..=dist as usize).map(|i| (x, y + i as i32)).collect()),
            &Step::Down(dist) => buf.push((1..=dist as usize).map(|i| (x, y - i as i32)).collect()),
        };
    }
    buf
}

pub fn build_steps(input: &str) -> Vec<Step> {
    input.split(",").filter_map(|s| s.parse().ok()).collect()
}

#[cfg(test)]
mod day03_1_test {
    use super::{build_steps, Step};
    use crate::crossed_wires::{calc_distance, plot_course};

    #[test]
    fn test_build_steps() {
        let input = "R7,L66,D123,U11,R-12";
        let steps = build_steps(input);
        assert_eq!(
            steps.as_slice(),
            &[
                Step::Right(7),
                Step::Left(66),
                Step::Down(123),
                Step::Up(11),
                Step::Right(-12),
            ],
        )
    }

    #[test]
    fn test_plot_course() {
        assert_eq!(
            plot_course(&[Step::Right(2)]).as_slice(),
            &[vec![(1, 0), (2, 0)]],
        );
        assert_eq!(plot_course(&[Step::Down(1)]).as_slice(), &[vec![(0, -1)]],);
        assert_eq!(
            plot_course(&[Step::Down(1), Step::Right(2)]).as_slice(),
            &[vec![(0, -1),], vec![(1, -1), (2, -1)]],
        );
        assert_eq!(
            plot_course(&[Step::Left(2), Step::Up(3)]).as_slice(),
            &[vec![(-1, 0), (-2, 0),], vec![(-2, 1), (-2, 2), (-2, 3)]],
        );
        assert_eq!(
            plot_course(&[Step::Up(2), Step::Down(1)]).as_slice(),
            &[vec![(0, 1), (0, 2),], vec![(0, 1)]],
        );
    }

    #[test]
    fn test_example_1() {
        let input_a = build_steps("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let input_b = build_steps("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(calc_distance(&input_a, &input_b), 159);
    }

    #[test]
    fn test_example_2() {
        let input_a = build_steps("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let input_b = build_steps("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(calc_distance(&input_a, &input_b), 135);
    }
}

#[cfg(test)]
mod day03_2_test {
    use super::{build_steps, calc_fewest_steps};

    #[ignore]
    #[test]
    fn test_example_1() {
        let input_a = build_steps("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let input_b = build_steps("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(calc_fewest_steps(&input_a, &input_b), Some(610));
    }

    #[ignore]
    #[test]
    fn test_example_2() {
        let input_a = build_steps("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let input_b = build_steps("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(calc_fewest_steps(&input_a, &input_b), Some(410));
    }
}
