import time
import sqlite3
import json
from datetime import datetime


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
                    timestamp TEXT NOT NULL
                )
            """)
            conn.commit()

    def capture(self, func_name: str, args: tuple, start_ns: int, end_ns: int):
        # args_json = json.dumps([str(arg) for arg in args])
        timestamp = datetime.now().isoformat()

        with sqlite3.connect(self.db_path) as conn:
            conn.execute("""
                INSERT INTO function_calls
                (function_name, args, start_time, end_time, timestamp)
                VALUES (?, ?, ?, ?, ?)
            """, (func_name, "", start_ns, end_ns, timestamp))
            conn.commit()

DB = Database()


def capture(func):
    def wrapper(*args, **kwargs):
        start: int = time.time_ns()

        value = func(*args, **kwargs)

        end: int = time.time_ns()

        DB.capture(func.__name__, args, start, end)

        return value

    return wrapper
