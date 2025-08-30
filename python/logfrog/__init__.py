# from typing import Final

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
