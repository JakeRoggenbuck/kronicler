import time
import kronicler


@kronicler.capture
def first_function(i, a):
    print("Hello world")

    acc = 0

    for x in range(1000):
        acc += 1 + 2 + x + i

    time.sleep(0.10 + (i * 0.01))


if __name__ == "__main__":
    for i in range(20):
        first_function(i, "hey")
