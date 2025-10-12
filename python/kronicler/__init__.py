from .kronicler import Database, database_init

from typing import Final
import time
from os import getenv


# Create an ENV var for kronicler to be unset
KRONICLER_ENABLED = getenv("KRONICLER_ENABLED", "true").lower() in ("true", "1")

DB = Database(sync_consume=True)


def capture(func):
    if not KRONICLER_ENABLED:
        # Return the original function unchanged
        return func

    def wrapper(*args, **krawgs):
        # Use nano seconds because it's an int
        # def perf_counter_ns() -> int: ...
        start: int = time.perf_counter_ns()

        # TODO: Should I go through args manually here and only share ones that
        # are string, float, and int? This way I can actually store them
        # without having to do GIL in Rust, which would be very slow
        # https://github.com/JakeRoggenbuck/kronicler/issues/15
        #
        # for a in args:
        #   if isinstance(a, str):
        #       strings.append(a)
        value = func(*args, **krawgs)

        end: int = time.perf_counter_ns()

        DB.capture(func.__name__, args, start, end)

        return value

    return wrapper


def decorator_example(func):
    def wrapper():
        print("Kronicler start...")

        func()

        print("Kronicler end...")

    return wrapper


__all__: Final[list[str]] = ["kronicler"]
