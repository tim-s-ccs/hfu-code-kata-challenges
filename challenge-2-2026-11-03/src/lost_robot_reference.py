"""Reference solution for the Lost Robot challenge.

This is a SPOILER. It is kept separate from `lost_robot_challenge.py` on
purpose, so it doesn't give away the answer to anyone who hasn't had a go
at the real challenge stub yet. It's here as a plain, readable reference
to check your own solution against (or to translate into another
language, e.g. for Challenge 3's Rust version).

No tricks, no golfing - just the simplest version that reads like the
problem statement.
"""

from typing import Optional, Tuple

Position = Tuple[int, int]

# Each instruction moves the robot one step in that direction.
_DELTAS = {
    "N": (0, 1),
    "S": (0, -1),
    "E": (1, 0),
    "W": (-1, 0),
}


def lost_robot(instructions: str) -> Tuple[Position, int, int, Optional[Position]]:
    x, y = 0, 0

    # The starting square counts as "visited" from the outset.
    visited = {(x, y)}

    furthest = 0
    first_revisit: Optional[Position] = None

    for move in instructions:
        dx, dy = _DELTAS[move]
        x += dx
        y += dy
        position = (x, y)

        # "Distance from home" is the minimum number of moves back to
        # (0, 0), i.e. Manhattan distance.
        distance_from_home = abs(x) + abs(y)
        furthest = max(furthest, distance_from_home)

        # Only remember the *first* position that gets visited again.
        if first_revisit is None and position in visited:
            first_revisit = position

        visited.add(position)

    final_position = (x, y)
    unique_positions = len(visited)

    return final_position, furthest, unique_positions, first_revisit


if __name__ == "__main__":
    # Quick manual sanity check matching the example in CHALLENGE.md.
    print(lost_robot("NNEESW"))
    # -> ((1, 1), 4, 7, None)
