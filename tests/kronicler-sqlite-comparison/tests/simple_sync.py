import kronicler_sqlite
import kronicler
import time
import json
import tqdm


WARMUP_COUNT = 10
CAPTURE_COUNT = 1000
REPEATS = 500


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


def foo_3():
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


def test_no_logging():
    ## Test for no logging

    # Warmup
    for _ in range(WARMUP_COUNT):
        foo_3()

    # Test
    for _ in range(CAPTURE_COUNT):
        foo_3()


def avg_sqlite() -> float:
    DB = kronicler_sqlite.Database()

    avg = DB.average("foo_1")
    return avg


def avg_columnar() -> float:
    DB = kronicler.Database()

    avg = DB.average("foo_2")
    return avg


def avg_no_logging() -> float:
    return 0


if __name__ == "__main__":
    insert_times_data = []
    avg_times_data = []

    for x in tqdm.tqdm(range(REPEATS)):
        # TEST sqlite inserts
        start = time.time_ns()
        test_sqlite()
        end = time.time_ns()
        print(f"{test_sqlite.__name__} took {end - start}ns")
        insert_times_data.append((test_sqlite.__name__, end - start))

        # TEST sqlite avg
        start = time.time_ns()
        avg_sqlite()
        end = time.time_ns()
        print(f"{avg_sqlite.__name__} took {end - start}ns")
        avg_times_data.append((avg_sqlite.__name__, end - start))

        # TEST columnar inserts
        start = time.time_ns()
        test_columnar()
        end = time.time_ns()
        print(f"{test_columnar.__name__} took {end - start}ns")
        insert_times_data.append((test_columnar.__name__, end - start))

        # TEST columnar avg
        start = time.time_ns()
        avg_columnar()
        end = time.time_ns()
        print(f"{avg_columnar.__name__} took {end - start}ns")
        avg_times_data.append((avg_columnar.__name__, end - start))

        # TEST no log inserts
        start = time.time_ns()
        test_no_logging()
        end = time.time_ns()
        print(f"{test_no_logging.__name__} took {end - start}ns")
        insert_times_data.append((test_no_logging.__name__, end - start))

        # TEST no logging avg
        start = time.time_ns()
        avg_no_logging()
        end = time.time_ns()
        print(f"{avg_no_logging.__name__} took {end - start}ns")
        avg_times_data.append((avg_no_logging.__name__, end - start))

    with open("sync_insert_data.json", "w") as file:
        json.dump(insert_times_data, file)

    with open("sync_avg_data.json", "w") as file:
        json.dump(avg_times_data, file)
