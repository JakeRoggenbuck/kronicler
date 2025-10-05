import kronicler


kronicler.DB = kronicler.Database(sync_consume=True)


@kronicler.capture
def first_function():
    print("Hello world")


if __name__ == "__main__":
    for i in range(1000):
        first_function()
