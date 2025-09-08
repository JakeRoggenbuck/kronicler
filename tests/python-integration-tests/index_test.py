import kronicler


@kronicler.capture
def first_function():
    print("Hello world")


@kronicler.capture
def second_function():
    print("Hello two")


if __name__ == "__main__":
    for i in range(10):
        first_function()

    for i in range(10):
        second_function()

    for i in range(10):
        first_function()
