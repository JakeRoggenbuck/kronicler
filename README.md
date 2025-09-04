# Kronicler

Automatic performance capture and analysis for production applications in Python using a custom columnar database written in Rust.

[![Rust](https://img.shields.io/badge/Rust-1A5D8A?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/JakeRoggenbuck?tab=repositories&q=&type=&language=rust&sort=stargazers)
[![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)](https://github.com/JakeRoggenbuck?tab=repositories&q=&type=&language=python&sort=stargazers)

## Why use Kronicler?

If you want to monitor the performance of a production application, kronicler offers efficient and lightweight logging with a single library.

Kronicler lets you view runtime statistics for functions like mean and median as well as statistics for different percentiles.

A use-case for these statistics is to find functions that occasionally operate much slower than they do on average. By looking at the "slowest" speed, the standard error, and the mean, you can find functions that occasionally run much slower than expected. Sometimes it's hard to find and replicate these issues in a test environment, so keeping logs in your production application can improve your ability to find these issues.

## Install (Python)

#### Install with Pip for Python

```sh
pip install kronicler
```

## Usage (Python)

Kronicler provides a Python decorator called `capture` that will calculate the time it takes to run the function.

```python
import kronicler

@kronicler.capture()
def my_function():
	pass
```

<!-- COMING SOON

## Install (Rust)

#### Install with Cargo for Rust

```
cargo install kronicler
```

Add as a dependency in your `Cargo.toml`.

```toml
[dependencies]
kronicler = "0.1.0"
```

-->

<!-- COMING SOON
## Usage in Rust

```rs
use kronicler;
```
-->

## Future Languages

Install and Usage for Rust is coming soon...

## Architecture

Simplified version of the package and database architecture. The data is passed from the Python decorator called [`capture`](https://github.com/JakeRoggenbuck/kronicler/blob/main/python/kronicler/__init__.py) to the [`database`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/database.rs)'s [`queue`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/queue.rs). It then consumes that [`queue`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/queue.rs) to insert each field into its respective [`column`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/column.rs). The [`column`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/column.rs) uses the [`bufferpool`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/bufferpool.rs) to operate on pages.

![System Architecture Dark Mode](./images/system-arch-dark-mode.svg#gh-dark-mode-only)
![System Architecture Light Mode](./images/system-arch-light-mode.svg#gh-light-mode-only)

This does not include details on:
- How the [`bufferpool`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/bufferpool.rs) manages [`pages`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/page.rs).
- How [`pages`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/page.rs) operate.
- [`capture`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/capture.rs), [`index`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/index.rs), [`row`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/row.rs), or saving and loading with [`metadata`](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/metadata.rs).

#### The Database

The columnar database is somewhat inspired by my previous database called [RedoxQL](https://github.com/JakeRoggenbuck/RedoxQL). A lot of the structure and architecture is different as well as how data is stored.

#### The Bufferpool

The bufferpool is based on my [bufferpool](https://github.com/JakeRoggenbuck/bufferpool) project. I had to modify it to work with the rest of this database.

#### The Columns

## Performance

## Analysis

<!--

Zen of Kronicler:

- Writes take as little time as possible
- Data analysis are quick on huge amounts data
- Adding logging is trivial
- It’s cheap to store billions of logs

### 1. Automatic logging

Make it as easy as possible to add logging to a function or backend endpoint:

```python
@kronicler.capture()
def my_database_function():
	pass
```

### 2. Logging is fast and negligible to performance (even in production)

The logging functions will be super fast and asynchronously send data to a queue to be written. Kronicler includes its own columnar database to perform this super fast logging.

> [!NOTE]
> I could separate the database and the rest of Kronicler to use the database for other stuff too. The database for this was the initial idea. I will build it as a monorepo for now and rebrand later if needed.

#### Queue System

The queue is quick to send logs to and they get processes quickly and asynchronously. The logs will include the function name, the time, and any other JSON object you want to include.

### 3. Simple to view logs

You can view your analytics from a web dashboard, so you don’t need to log in to a server to see the analytics through a terminal.

#### Web Portal

You can view tables and graphs of the data and look at averages, anomalies, p90, p99, etc.

-->

<!-- :frog: -->
