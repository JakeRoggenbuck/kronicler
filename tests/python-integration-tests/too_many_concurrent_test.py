import kronicler


kronicler.database_init()


@kronicler.capture
def first_function():
    print("Hello world")


if __name__ == "__main__":
    for i in range(1000):
        first_function()

# TODO: Make a kronicler.cleanup method that calls the consume one last time.
import time

time.sleep(1)
