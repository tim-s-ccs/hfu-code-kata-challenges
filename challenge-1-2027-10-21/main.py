import timeit
from pathlib import Path

from src.anagaram_challenge import anagaram_challenge


DIR = Path(__file__).resolve().parent
WORD_LIST_PATH = DIR / "data" / "word_list.txt"

NUMBER_OF_ITERATIONS = 10

def main():
    print("Running the anagaram_challenge function...")

    result = anagaram_challenge(WORD_LIST_PATH)

    if result:
        print(f"Number of anagarams: {result[0]}")
        print(f"Longest anagaram length: {len(result[1][0])} - list: {', '.join(result[1])}")
        print(f"Most common anagarams length: {len(result[2])} - list: {', '.join(result[2])}")
        print("Are you right?")

        print("Benchmarking...")

        duration = timeit.timeit('anagaram_challenge(WORD_LIST_PATH)', globals=globals(), number=NUMBER_OF_ITERATIONS)
        print(f"Average: {duration / NUMBER_OF_ITERATIONS:.4f} seconds per call")
    else:
        print("No result for anagarams...")

    print("Finish!!!")

if __name__ == "__main__":
    main()