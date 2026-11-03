# Lost Robot 🤖

**Level: `EASY`**

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

Your challenge this week is to fill out the `lost_robot` function.

It should return:

- The robot's final position
- The furthest the robot travelled from its starting position
- The number of unique positions the robot visited
- The first position that the robot visited more than once

For example:

```python
lost_robot("NNEESW")
```

should tell us that the robot:

- Ended at `(1, 1)`
- Got as far as `4` moves away from home
- Visited `7` unique positions
- Did not visit the same position twice

## Distance

For this challenge, distance means the **minimum number of moves the robot would need to make to get back to `(0, 0)`**.

For example:

```text
(3, 2)
```

is `5` moves away from home.

## Revisited positions

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

Feel free to write tests for your code, but if you want a quick check, you can run:

```shell
uv run main.py
```

which will print your result and run some example journeys.

## ⭐ Bonus challenges

Finished already?

Try extending your solution to find:

- The position the robot visited the most
- How many times the robot returned home
- The longest the robot travelled without visiting the same position twice

Good luck! 🤖