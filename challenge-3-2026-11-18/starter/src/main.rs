//! Runs `lost_robot` against `../data/robot_test_cases.csv` and reports
//! pass/fail per row, the same way `challenge-2-2026-11-03/main.py` does
//! for the Python version. Run with `cargo run` from inside `starter/`.

use lost_robot_rust::lost_robot;
use std::env;
use std::fs;
use std::panic;
use std::path::PathBuf;

/// Parses a CSV field that may be empty (meaning "no value").
fn parse_optional_i64(field: &str) -> Option<i64> {
    let trimmed = field.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.parse().expect("expected an integer"))
    }
}

fn main() {
    // The test data has no quoted/embedded commas, so a plain split is
    // enough - no need to pull in a CSV crate for this.
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let data_path: PathBuf = PathBuf::from(manifest_dir)
        .join("..")
        .join("data")
        .join("robot_test_cases.csv");

    let contents = fs::read_to_string(&data_path)
        .unwrap_or_else(|err| panic!("couldn't read {}: {err}", data_path.display()));

    println!("Running the lost_robot function against the test cases...");

    // Silence the default panic printout - if `lost_robot` is still
    // `todo!()` (or otherwise panics), we report it as [ERROR] per row
    // instead of spamming a full backtrace for every test case.
    panic::set_hook(Box::new(|_| {}));

    let mut lines = contents.lines();
    lines.next(); // skip header row

    let mut passed = 0usize;
    let mut total = 0usize;

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        let fields: Vec<&str> = line.split(',').collect();
        assert_eq!(fields.len(), 8, "unexpected row shape: {line:?}");

        let name = fields[0];
        let instructions = fields[1];
        let expected_final_x: i64 = fields[2].parse().expect("expected_final_x");
        let expected_final_y: i64 = fields[3].parse().expect("expected_final_y");
        let expected_furthest: i64 = fields[4].parse().expect("expected_furthest_distance");
        let expected_unique: usize = fields[5].parse().expect("expected_unique_positions");
        let expected_revisit = match (
            parse_optional_i64(fields[6]),
            parse_optional_i64(fields[7]),
        ) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => None,
        };
        let expected = (
            (expected_final_x, expected_final_y),
            expected_furthest,
            expected_unique,
            expected_revisit,
        );

        total += 1;

        match panic::catch_unwind(|| lost_robot(instructions)) {
            Ok(actual) if actual == expected => {
                println!("[PASS]  {name:?}");
                passed += 1;
            }
            Ok(actual) => {
                println!("[FAIL]  {name:?} (instructions={instructions:?})");
                println!("        expected: {expected:?}");
                println!("        actual:   {actual:?}");
            }
            Err(_) => {
                println!("[ERROR] {name:?} (instructions={instructions:?}): lost_robot panicked (not implemented yet?)");
            }
        }
    }

    println!();
    println!("{passed}/{total} test cases passed ({} failed)", total - passed);
}
