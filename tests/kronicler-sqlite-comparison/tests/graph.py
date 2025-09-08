import matplotlib.pyplot as plt
import json


with open("insert_data.json") as file:
    data = json.load(file)

# Separate data
sqlite_times = [t for name, t in data if name == "test_sqlite"]
columnar_times = [t for name, t in data if name == "test_columnar"]
runs = list(range(1, len(sqlite_times) + 1))

# Convert ns to ms
sqlite_times_ms = [t / 1e6 for t in sqlite_times]
columnar_times_ms = [t / 1e6 for t in columnar_times]

# Plot
plt.figure(figsize=(8, 5))
plt.plot(runs, sqlite_times_ms, marker="o", label="SQLite")
plt.plot(runs, columnar_times_ms, marker="o", label="Columnar")
plt.title("Benchmark Results")
plt.xlabel("Run #")
plt.ylabel("Time (ms)")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.show()
