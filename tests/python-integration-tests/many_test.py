import kronicler

# If you want a sync consume database
kronicler.DB = kronicler.Database(sync_consume=True)


@kronicler.capture
def first_function():
    print("Hello world")


if __name__ == "__main__":
    for i in range(10):
        first_function()
