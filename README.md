![Kronicler](./images/Kronicler-logo.svg)

Automatic performance capture and analysis for production applications in Python using a custom columnar database written in Rust.

[![Rust](https://img.shields.io/badge/Rust-1A5D8A?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/JakeRoggenbuck?tab=repositories&q=&type=&language=rust&sort=stargazers)
[![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)](https://github.com/JakeRoggenbuck?tab=repositories&q=&type=&language=python&sort=stargazers)
[![Rust](https://img.shields.io/github/actions/workflow/status/jakeroggenbuck/kronicler/CI.yml?branch=main&style=for-the-badge)](https://github.com/JakeRoggenbuck/kronicler/actions)
[![Version](https://img.shields.io/pypi/v/kronicler?style=for-the-badge)](https://pypi.org/project/kronicler)
[![Crates Version](https://img.shields.io/crates/v/kronicler?style=for-the-badge)](https://crates.io/crates/kronicler)
[![Downloads](https://img.shields.io/crates/d/kronicler?style=for-the-badge)](https://crates.io/crates/kronicler)

View Kronicler on [PyPi.org](https://pypi.org/project/kronicler), [Crates.io](https://crates.io/crates/kronicler) and [GitHub](https://github.com/JakeRoggenbuck/kronicler).

> [!IMPORTANT]
> Kronicler is still early in development! Currently you can install and try out the [logging](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#logging). [Analysis](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analysis) features are coming soon.

## Benefits of using Kronicler

- Automatic performance capturing
- Lightweight and concurrent\*
- One Python dependency
- Works out-of-the-box without configuration 

\* concurrency is in development but not fully implemented as of version 0.1.0. Track concurrency in [issue #41](https://github.com/JakeRoggenbuck/kronicler/issues/41).

## Why use Kronicler?

If you want to monitor the performance of a production application, kronicler offers efficient and lightweight logging with a single library. Kronicler lets you view runtime statistics for functions like mean and median as well as statistics for different percentiles.

A use-case for these statistics is to find functions that occasionally operate much slower than they do on average. By looking at the "slowest" speed, the standard error, and the mean, you can find functions that occasionally run much slower than expected. Sometimes it's hard to find and replicate these issues in a test environment, so keeping logs in your production application can improve your ability to find these issues.

## What really is Kronicler


| Name                     | Description                                                                                                                                        | Link                                                                                                                           |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **Kronicler (Python)**   | A [Python library](https://pypi.org/project/kronicler/) that provides the `@kronicler.capture` decorator to save performance logs to a database.   | [More about Python](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#install-python)                             |
| **Kronicler (Database)** | Custom columnar database designed for log capturing and performance analysis.                                                                      | [More about Database](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#architecture)                             |
| **Kronicler (Rust)**     | A [Rust library](https://crates.io/crates/kronicler) for accessing the columnar database and capture functions for Rust-based log capturing tasks. | [More about Rust](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#using-kroniclers-database-directly)           |
| **Kronicler (CLI)**      | Rust-based CLI tool for analyzing performance logs captured with the capture methods.                                                              | [More about CLI](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analytics-cli)                                 |
| **Kronicler (Web)**      | Prototype web portal for remotely viewing performance logs.                                                                                        | [More about Web](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analysis-web-dashboard)                        |

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

## Future Languages

Install and Usage for Rust is coming soon...

I plan to implement the Rust version as an `attribute` to be used like the following:

```rust
#[capture]
fn foo() { todo!() }
```

## Examples

#### Using Kronicler (Basic Example)

Here is the most basic usage of Kronicler. Use it to capture the runtime of functions by adding the `@kronicler.capture` decorator.

```python
import kronicler

@kronicler.capture
def foo():
    print("Foo")
```

#### Using Kronicler with FastAPI

With just two lines of code, you can add Kronicler to your [FastAPI](https://fastapi.tiangolo.com) server.

```python
from fastapi import FastAPI
import uvicorn
import kronicler

app = FastAPI()


# You need to wrap helper functions
@kronicler.capture
def foo():
    return {"Hello": "World"}

# You cannot wrap routes right now
@app.get("/")
def read_root():
    return foo()

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
```

Code from [tests/fastapi-test/main.py](https://github.com/JakeRoggenbuck/kronicler/blob/main/tests/fastapi-test/main.py).

## Using Kronicler manually

It's recommended to first use Kronicler's build-in `capture` decorator. However, if you want to write your own capture statements and fetch data yourself, you can use the functions in that come with the Python library.

```python
import kronicler

DB = kronicler.Database()

def foo():
    DB.capture("String Value", [], 100, 200)

fetched = DB.fetch(0)
print(fetched)
```

More functionality will be added to the Python library in future releases.

## Using Kronicler's database directly

If you're interested in using Kronicler's database directly in Rust to add custom logging functions (or just to use a columnar database), the library is published to [crates.io](https://crates.io/crates/kronicler).

#### Install with Cargo for Rust

```
cargo install kronicler
```

Add as a dependency in your `Cargo.toml`.

```toml
[dependencies]
kronicler = "0.1.0"
```

To get a good idea of how to use Kronicler's internal Rust database, I'd recommended looking at some of the tests in the Rust files. You can also look at the source code for the `kr` binary in [main.rs](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/bin/main.rs).

Here is an example of a function that fetches data based on index. It creates a `Database` from the `new_reader` trait.

```rs
use kronicler::database::Database;

fn fetch_one(index: usize) {
    let mut db = Database::new_reader();

    let row = db.fetch(index);

    if let Some(r) = row {
        println!("{}", r.to_string());
    }
}

fn main() {
    fetch_one(0);
    fetch_one(1);
}
```

You can also make a database that writes new capture data with the `new` trait.

```rs
use kronicler::database::Database;

fn main() {
    let mut db = Database::new();

    db.capture("Name".to_string(), vec![], 100, 200);
}
```

## Performance

Kronicler is designed to be as lightweight as possible. By adding logs to a queue concurrently\*, Kronicler doesn't affect performance by much \[citation needed\].

For accessing logs and running calculations, Kronicler uses a columnar database design to optimize file operations when looking at lots of data from only a few columns typical of analytics tasks.

\* concurrency is in development but not fully implemented as of version 0.1.0. Track concurrency in [issue #41](https://github.com/JakeRoggenbuck/kronicler/issues/41).

## Analysis Web Dashboard

The Analysis Web Dashboard is still under construction. This feature will let you remotely view the logs collected from Kronicler.

#### Mock-up

The Web Dashboard may look something like this. It will show important information about all of the functions at the top. It will also include a graph of the functions performance over time.

<img width="1442" height="885" alt="image" src="https://github.com/user-attachments/assets/69f03f71-9e94-473a-b309-9a13dae94ff5" />

This mock-up was created with Claude and the future dashboard may vary substantially.

## Analytics CLI

#### Install the Analytics CLI

```
cargo install kronicler
```

You can view all of your data by running `kr` in the directory of your data:

```
kr --fetch all
```

```
kr --fetch <index>
```

You should see the data collected:

<img width="1177" height="531" alt="image" src="https://github.com/user-attachments/assets/bd1d3867-b201-4d6d-9c00-9734536be7e4" />

In the future, there will be many options for sorting, filtering, and viewing specific statistics.

## Logging

By adding the `capture` decorator to your code (as seen below), Kronicler will automatically test the runtime of your function when it gets called. The results of the test get added to the database. This data can later be viewed in the [Analysis Web Dashboard](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analysis-web-dashboard) or the [Analytics CLI](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analytics-cli).

```python
import kronicler

@kronicler.capture()
def my_function():
	pass
```

## Development

#### Building the Python package

Build the package

```sh
maturin build
```

Install the package

```sh
pip install --force-reinstall target/wheels/kronicler-*
```

#### Testing

You can run the testing suite with the following command:

```
cargo test
```

The tests should all succeed

<img width="1157" height="377" alt="image" src="https://github.com/user-attachments/assets/aae0fb07-f9d5-4482-a06e-1e2e5e6eb320" />

<!-- :frog: -->
