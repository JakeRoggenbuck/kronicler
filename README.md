# LogFrog

Automatic performance analysis for production applications.

Zen of LogFrog:

- Writes take as little time as possible
- Data analysis are quick on huge amounts data
- Adding logging is trivial
- It’s cheap to store billions of logs

### 1. Automatic logging

Make it as easy as possible to add logging to a function or backend endpoint:

```python
@logfrog.capture()
def my_database_function():
	pass
```

### 2. Logging is fast and negligible to performance (even in production)

The logging functions will be super fast and asynchronously send data to a queue to be written. LogFrog includes its own columnar database to perform this super fast logging.

> [!NOTE]
> I could separate the database and the rest of LogFrog to use the database for other stuff too. The database for this was the initial idea. I will build it as a monorepo for now and rebrand later if needed.

#### Queue System

The queue is quick to send logs to and they get processes quickly and asynchronously. The logs will include the function name, the time, and any other JSON object you want to include.

### 3. Simple to view logs

You can view your analytics from a web dashboard, so you don’t need to log in to a server to see the analytics through a terminal.

#### Web Portal

You can view tables and graphs of the data and look at averages, anomalies, p90, p99, etc.
