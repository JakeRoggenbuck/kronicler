from .logfrog import LFQueue

from typing import Final
import time

# LFQueue will be a singleton
LFQ = LFQueue()


def capture(func):
    def wrapper(*args, **krawgs):
        # Use nano seconds because it's an int
        # def time_ns() -> int: ...
        start: int = time.time_ns()

        # TODO: Should I go through args manually here and only share ones that
        # are string, float, and int? This way I can actually store them
        # without having to do GIL in Rust, which would be very slow
        # https://github.com/JakeRoggenbuck/logfrog/issues/15
        #
        # for a in args:
        #   if isinstance(a, str):
        #       strings.append(a)
        func(*args, **krawgs)

        end: int = time.time_ns()

        LFQ.capture(func.__name__, args, start, end)

    return wrapper


def decorator_example(func):
    def wrapper():

        print("LogFrog start...")

        func()

        print("LogFrog end...")

    return wrapper


# Use when Rust is added
__all__: Final[list[str]] = [
    "logfrog"
]
