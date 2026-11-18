# Become a Rustacean 🦀

**Level: `MEDIUM`**

Same brain, new language. This week is **exactly the [Lost Robot](/challenge-2-2026-11-03/CHALLENGE.md) challenge from last week** - but you're solving it in **Rust**. The difficulty this time isn't the algorithm, it's the language.

## The problem

Oh no! Our robot has got lost...

Our robot starts at coordinates `(0, 0)` and has been given a series of instructions telling it which direction to move.

Each instruction moves the robot **one position**:

- `N` - North
- `E` - East
- `S` - South
- `W` - West

For example:

```text
NNEESW
```

would move the robot:

```text
(0, 0)
(0, 1)
(0, 2)
(1, 2)
(2, 2)
(2, 1)
(1, 1)
```

Your challenge this week is to write a function

```rust
fn lost_robot(instructions: &str) -> (Position, i64, usize, Option<Position>)
```

(where `Position` is some `(x, y)` pair) that returns:

- The robot's final position
- The furthest the robot travelled from its starting position
- The number of unique positions the robot visited
- The first position that the robot visited more than once

For example:

```rust
lost_robot("NNEESW")
```

should tell us that the robot:

- Ended at `(1, 1)`
- Got as far as `4` moves away from home
- Visited `7` unique positions
- Did not visit the same position twice

### Distance

For this challenge, distance means the **minimum number of moves the robot would need to make to get back to `(0, 0)`**.

For example:

```text
(3, 2)
```

is `5` moves away from home.

### Revisited positions

If the robot visits the same position more than once, return the **first position it revisited**.

For example:

```text
NESW
```

brings the robot back to:

```text
(0, 0)
```

So `(0, 0)` is the first position visited more than once.

If the robot never visits the same position twice, return `None`.

## Pick your track

However much of the Rust setup you want to fight yourself is up to you:

- **Standard** - use the scaffold in [`starter/`](/challenge-3-2026-11-18/starter). It's a ready-to-go Cargo project: fill in `lost_robot` in `starter/src/lib.rs`, then run

  ```shell
  cd challenge-3-2026-11-18/starter
  cargo run
  ```

  to check your solution against the test cases in [`data/robot_test_cases.csv`](/challenge-3-2026-11-18/data/robot_test_cases.csv) (the same ones Challenge 2 used). No external crates needed.

- **Hardcore** - ignore `starter/` entirely. Run `cargo new` yourself, decide your own types/structure, and write your own check against `data/robot_test_cases.csv`.

Either way, the test data and expected answers are identical to Challenge 2's - if you solved that one, you already know what "correct" looks like.

## ⭐ Bonus challenges

Finished already?

Try extending your solution to find:

- The position the robot visited the most
- How many times the robot returned home
- The longest the robot travelled without visiting the same position twice

Good luck, and welcome to the borrow checker! 🦀
