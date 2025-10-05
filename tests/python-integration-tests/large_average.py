import kronicler
from random import randint

DB = kronicler.Database(sync_consume=True)


def foo():
    for _ in range(1000):
        DB.capture(
            "String Value",
            [],
            randint(100, 200),
            randint(300, 400),
        )


foo()

fetched = DB.fetch(0)
print(fetched)
