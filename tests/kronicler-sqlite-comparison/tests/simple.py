import kronicler_sqlite
import kronicler
import time
import json


WARMUP_COUNT = 10
CAPTURE_COUNT = 1000
REPEATS = 5


@kronicler_sqlite.capture
def foo_1():
    val = 9

    for x in range(4):
        val += val + x

    return val


@kronicler.capture
def foo_2():
    val = 9

    for x in range(4):
        val += val + x

    return val


def test_sqlite():
    ## Test for kronicler_sqlite

    # Warmup
    for _ in range(WARMUP_COUNT):
        foo_1()

    # Test
    for _ in range(CAPTURE_COUNT):
        foo_1()


def test_columnar():
    ## Test for kronicler

    # Warmup
    for _ in range(WARMUP_COUNT):
        foo_2()

    # Test
    for _ in range(CAPTURE_COUNT):
        foo_2()


if __name__ == "__main__":
    data = []

    for x in range(REPEATS):

        start = time.time_ns()
        test_sqlite()
        end = time.time_ns()
        print(f"{test_sqlite.__name__} took {end - start}ns")
        data.append((test_sqlite.__name__, end - start))

        start = time.time_ns()
        test_columnar()
        end = time.time_ns()
        print(f"{test_columnar.__name__} took {end - start}ns")
        data.append((test_columnar.__name__, end - start))

    with open("data.json", "w") as file:
        json.dump(data, file)
