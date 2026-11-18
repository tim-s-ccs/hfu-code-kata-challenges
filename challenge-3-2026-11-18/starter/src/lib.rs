//! Lost Robot - see `../../CHALLENGE.md` for the full problem statement.
//!
//! Fill in `lost_robot` below. It should return:
//!
//! - the robot's final position
//! - the furthest the robot travelled from its starting position
//! - the number of unique positions the robot visited
//! - the first position the robot visited more than once (or `None`)

use std::collections::HashSet;
use std::collections::HashMap;

pub type Position = (i64, i64);

pub fn lost_robot(instructions: &str) -> (Position, i64, usize, Option<Position>) {
    let DELTAS: HashMap::<&str, Position> = HashMap::from([
        ("N", (0, 1)),
        ("S", (0, -1)),
        ("E", (1, 0)),
        ("W", (-1, 0)),
    ]);

    let x: i64 = 0;
    let y: i64 = 0;

    let mut visited: HashSet<Position> = HashSet::new();

    let initial_position: Position = (x, y);

    visited.insert(initial_position);

    let mut furthest = 0;

    let first_revisit: Position;

    for letter in instructions.chars() {
        let delta = DELTAS.get(&letter);
        println!("{} {}", delta[0], delta[1]);
    }

    // println!("{} {}", x, y);

    // println!("{}", initial_position.to_string());

    todo!("implement me - see ../../CHALLENGE.md")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_journey() {
        // From CHALLENGE.md: "NNEESW" should end at (1, 1), 4 moves from
        // home at its furthest, having visited 7 unique positions, with
        // no position visited twice.
        assert_eq!(lost_robot("NNEESW"), ((1, 1), 4, 7, None));
    }
}
