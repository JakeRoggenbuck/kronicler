import kronicler


@kronicler.capture
def first_function():
    print("Hello world")

if __name__ == "__main__":
    for i in range(1000):
        first_function()
