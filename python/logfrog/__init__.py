from .logfrog import LFQueue

from typing import Final
import time

# LFQueue will be a singleton
LFQ = LFQueue()


def capture(func):
    def wrapper():
        start = time.time()

        func()

        end = time.time()

        LFQ.capture(start, end)

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
