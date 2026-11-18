//! Lost Robot - see `../../CHALLENGE.md` for the full problem statement.
//!
//! Fill in `lost_robot` below. It should return:
//!
//! - the robot's final position
//! - the furthest the robot travelled from its starting position
//! - the number of unique positions the robot visited
//! - the first position the robot visited more than once (or `None`)

pub type Position = (i64, i64);

pub fn lost_robot(instructions: &str) -> (Position, i64, usize, Option<Position>) {
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
