import kronicler
from random import randint
import time

DB = kronicler.Database(sync_consume=True)


def mean(data: list):
    return sum(data) / len(data)


def foo():
    for _ in range(100_000):
        a = randint(100, 200)
        b = randint(300, 400)

        DB.capture("jake", [], a, b)


start = time.time()
foo()
total = time.time() - start
print(f"Running insert took {total}")


fetched = DB.fetch(0)
print("First row:", fetched)

fetched = DB.fetch_all()
print(f"Fetched all {len(fetched)} rows.")

start = time.time()
kr_avg = DB.average("jake")
kr_avg_time = time.time() - start
print("Kronicler average: ", kr_avg, f"ran in {kr_avg_time}.")
