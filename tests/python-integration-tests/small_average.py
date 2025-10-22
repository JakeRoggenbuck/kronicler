import kronicler
from random import randint
import time

DB = kronicler.Database(sync_consume=True)

# This version of the script computes a ground truth in memory. This doesn't
# scale super well and costs a lot of memory.
# The test ./large_average_no_in_mem_comp.py does not do this, so you can
# compare the speed of this test against that one.
ground_truth_data = []


def mean(data: list):
    return sum(data) / len(data)


def foo():
    for _ in range(1000):
        a = randint(100, 200)
        b = randint(300, 400)

        DB.capture("jake", [], a, b)

        ground_truth_data.append(b - a)


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

start = time.time()
py_avg = mean(ground_truth_data)
py_avg_time = time.time() - start
print("Ground truth average: ", py_avg, f"ran in {py_avg_time}.")

assert kr_avg - py_avg < 0.0001
