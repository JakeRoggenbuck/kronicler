import matplotlib.pyplot as plt
import json

with open("insert_data.json") as file:
    insert_data = json.load(file)

# Separate data
sqlite_times = [t for name, t in insert_data if name == "test_sqlite"]
columnar_times = [t for name, t in insert_data if name == "test_columnar"]
no_logging_times = [t for name, t in insert_data if name == "test_no_logging"]

runs = list(range(1, len(sqlite_times) + 1))

# Convert ns to ms
sqlite_times_ms = [t / 1e6 for t in sqlite_times]
columnar_times_ms = [t / 1e6 for t in columnar_times]
no_logging_times_ms = [t / 1e6 for t in no_logging_times]

# Plot Insert benchmark with log scale
plt.figure(figsize=(8, 5))
plt.plot(runs, sqlite_times_ms, marker="o", label="SQLite")
plt.plot(runs, columnar_times_ms, marker="o", label="Columnar")
plt.plot(runs, no_logging_times_ms, marker="o", label="No Logging")
plt.yscale("log")  # <-- Logarithmic scale
plt.title("Insert Benchmark Results (Log Scale)")
plt.xlabel("Run #")
plt.ylabel("Time (ms, log scale)")
plt.legend()
plt.grid(True, which="both", ls="--")  # grid for log scale
plt.tight_layout()
plt.savefig(f"insert_log_{len(runs)}.png")
plt.show()


with open("avg_data.json") as file:
    avg_data = json.load(file)

# Separate data
sqlite_times = [t for name, t in avg_data if name == "avg_sqlite"]
columnar_times = [t for name, t in avg_data if name == "avg_columnar"]
no_logging_times = [t for name, t in avg_data if name == "avg_no_logging"]

runs = list(range(1, len(sqlite_times) + 1))

# Convert ns to ms
sqlite_times_ms = [t / 1e6 for t in sqlite_times]
columnar_times_ms = [t / 1e6 for t in columnar_times]
no_logging_times_ms = [t / 1e6 for t in no_logging_times]

# Plot Average benchmark with log scale
plt.figure(figsize=(8, 5))
plt.plot(runs, sqlite_times_ms, marker="o", label="SQLite")
plt.plot(runs, columnar_times_ms, marker="o", label="Columnar")
plt.plot(runs, no_logging_times_ms, marker="o", label="No Logging")
plt.yscale("log")  # <-- Logarithmic scale
plt.title("Average Function Benchmark Results (Log Scale)")
plt.xlabel("Run #")
plt.ylabel("Time (ms, log scale)")
plt.legend()
plt.grid(True, which="both", ls="--")
plt.tight_layout()
plt.savefig(f"avg_log_{len(runs)}.png")
plt.show()
