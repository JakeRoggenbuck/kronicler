import kronicler_sqlite
import kronicler
import time


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
    for _ in range(10):
        foo_1()

    # Test
    for _ in range(10_000):
        foo_1()


def test_columnar():
    ## Test for kronicler

    # Warmup
    for _ in range(10):
        foo_1()

    # Test
    for _ in range(10_000):
        foo_1()


if __name__ == "__main__":
    for x in range(5):

        start = time.time_ns()
        test_columnar()
        end = time.time_ns()
        print(f"{test_columnar.__name__} took {end - start}ns")

        start = time.time_ns()
        test_sqlite()
        end = time.time_ns()
        print(f"{test_sqlite.__name__} took {end - start}ns")
