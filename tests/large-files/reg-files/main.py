import kronicler


@kronicler.capture
def my_function():
	return 1


@kronicler.capture
def my_function2():
	return 2


if __name__ == "__main__":
    for x in range(30_000):
        my_function()

    for x in range(30_000):
        my_function()
