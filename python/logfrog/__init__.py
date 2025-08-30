# from typing import Final
# from .logfrog import LFQueue
import time

# TODO: Define
# LFQueue will be a singleton
# LFQ = LFQueue()


def capture(func):
    def wrapper():
        start = time.time()

        func()

        end = time.time()

        # TODO: Internally add start and end to LogFrog queue
        # LFQ.add(start, end)

    return wrapper


def decorator_example(func):
    def wrapper():

        print("LogFrog start...")

        func()

        print("LogFrog end...")

    return wrapper


# Use when Rust is added
# __all__: Final[list[str]] = [
#     "logfrog"
# ]
