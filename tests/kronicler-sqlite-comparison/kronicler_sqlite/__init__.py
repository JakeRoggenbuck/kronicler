import time
import sqlite3


class Database:
    def __init__(self, db_path: str = "database.db"):
        self.db_path = db_path

        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                CREATE TABLE IF NOT EXISTS function_calls (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    function_name TEXT NOT NULL,
                    args TEXT,
                    start_time INTEGER NOT NULL,
                    end_time INTEGER NOT NULL,
                    delta INTEGER NOT NULL
                )
            """)
            conn.commit()

    def capture(self, func_name: str, args: tuple, start_ns: int, end_ns: int):
        delta = end_ns - start_ns

        with sqlite3.connect(self.db_path) as conn:
            conn.execute(
                """
                INSERT INTO function_calls
                (function_name, args, start_time, end_time, delta)
                VALUES (?, ?, ?, ?, ?)
            """,
                (func_name, "", start_ns, end_ns, delta),
            )
            conn.commit()

    def average(self, function_name: str) -> float:
        """Return the average delta in nanoseconds for a specific function."""
        with sqlite3.connect(self.db_path) as conn:
            result = conn.execute(
                """
                SELECT AVG(delta)
                FROM function_calls
                WHERE function_name = ?
            """,
                (function_name,),
            ).fetchone()[0]
            return result if result is not None else 0.0


DB = Database()


def capture(func):
    def wrapper(*args, **kwargs):
        start: int = time.time_ns()

        value = func(*args, **kwargs)

        end: int = time.time_ns()

        DB.capture(func.__name__, args, start, end)

        return value

    return wrapper
