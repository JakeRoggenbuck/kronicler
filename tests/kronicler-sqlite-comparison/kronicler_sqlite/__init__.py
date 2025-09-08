import time


def capture(func):
    def wrapper(*args, **krawgs):
        start: int = time.time_ns()

        func(*args, **krawgs)

        end: int = time.time_ns()

        # DB.capture(func.__name__, args, start, end)

    return wrapper
