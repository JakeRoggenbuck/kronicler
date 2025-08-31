from .logfrog import LFQueue

from typing import Final
import time

# LFQueue will be a singleton
LFQ = LFQueue()


def capture(func):
    def wrapper():
        # Use nano seconds because it's an int
        # def time_ns() -> int: ...
        start: int = time.time_ns()

        func()

        end: int = time.time_ns()

        LFQ.capture(func.__name__, start, end)

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
