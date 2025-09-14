import matplotlib.pyplot as plt
import json

# Toggle to show or hide the "No Logging" series in non-log plots
show_no_logging = False

# --- Insert Data ---
with open("concurrent_insert_data.json") as file:
    insert_data = json.load(file)

sqlite_times = [t for name, t in insert_data if name == "test_sqlite"]
columnar_times = [t for name, t in insert_data if name == "test_columnar"]
no_logging_times = [t for name, t in insert_data if name == "test_no_logging"]

runs = list(range(1, len(sqlite_times) + 1))

# Convert ns to ms
sqlite_times_ms = [t / 1e6 for t in sqlite_times]
columnar_times_ms = [t / 1e6 for t in columnar_times]
no_logging_times_ms = [t / 1e6 for t in no_logging_times]

# Plot Insert benchmark
plt.figure(figsize=(8, 5))
plt.plot(runs, sqlite_times_ms, marker="o", label="SQLite")
plt.plot(runs, columnar_times_ms, marker="o", label="Columnar")
if show_no_logging:
    plt.plot(runs, no_logging_times_ms, marker="o", label="No Logging")
plt.title("Insert Benchmark Results")
plt.xlabel("Run #")
plt.ylabel("Time (ms)")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig(f"concurrent_insert_{len(runs)}.png")
plt.show()


# --- Average Data ---
with open("concurrent_avg_data.json") as file:
    avg_data = json.load(file)

sqlite_times = [t for name, t in avg_data if name == "avg_sqlite"]
columnar_times = [t for name, t in avg_data if name == "avg_columnar"]
no_logging_times = [t for name, t in avg_data if name == "avg_no_logging"]

runs = list(range(1, len(sqlite_times) + 1))

# Convert ns to ms
sqlite_times_ms = [t / 1e6 for t in sqlite_times]
columnar_times_ms = [t / 1e6 for t in columnar_times]
no_logging_times_ms = [t / 1e6 for t in no_logging_times]

# Plot Average benchmark
plt.figure(figsize=(8, 5))
plt.plot(runs, sqlite_times_ms, marker="o", label="SQLite")
plt.plot(runs, columnar_times_ms, marker="o", label="Columnar")
if show_no_logging:
    plt.plot(runs, no_logging_times_ms, marker="o", label="No Logging")
plt.title("Average Function Benchmark Results")
plt.xlabel("Run #")
plt.ylabel("Time (ms)")
plt.legend()
plt.grid(True)
plt.tight_layout()
plt.savefig(f"concurrent_avg_{len(runs)}.png")
plt.show()
