import csv
from pathlib import Path

from src.lost_robot_challenge import lost_robot


DIR = Path(__file__).resolve().parent
TEST_CASES_PATH = DIR / "data" / "robot_test_cases.csv"


def _parse_optional_int(value: str):
    return int(value) if value else None


def _parse_expected(row: dict):
    final_position = (int(row["expected_final_x"]), int(row["expected_final_y"]))
    furthest_distance = int(row["expected_furthest_distance"])
    unique_positions = int(row["expected_unique_positions"])

    first_revisited_x = _parse_optional_int(row["expected_first_revisited_x"])
    first_revisited_y = _parse_optional_int(row["expected_first_revisited_y"])
    if first_revisited_x is None or first_revisited_y is None:
        first_revisited = None
    else:
        first_revisited = (first_revisited_x, first_revisited_y)

    return (final_position, furthest_distance, unique_positions, first_revisited)


def main():
    print("Running the lost_robot function against the test cases...")

    with open(TEST_CASES_PATH, newline="", encoding="utf-8") as f:
        rows = list(csv.DictReader(f))

    passed = 0
    failed = 0

    for row in rows:
        name = row["name"]
        instructions = row["instructions"]
        expected = _parse_expected(row)

        try:
            actual = lost_robot(instructions)
        except Exception as exc:  # noqa: BLE001 - we want to report any failure, not crash the run
            print(f"[ERROR] {name!r} (instructions={instructions!r}): {exc}")
            failed += 1
            continue

        if actual is None:
            print(f"[SKIP]  {name!r}: lost_robot hasn't been implemented yet")
            failed += 1
            continue

        if tuple(actual) == expected:
            print(f"[PASS]  {name!r}")
            passed += 1
        else:
            print(f"[FAIL]  {name!r} (instructions={instructions!r})")
            print(f"        expected: {expected}")
            print(f"        actual:   {tuple(actual)}")
            failed += 1

    print()
    print(f"{passed}/{len(rows)} test cases passed ({failed} failed)")


if __name__ == "__main__":
    main()
