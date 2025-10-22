![Kronicler](https://raw.githubusercontent.com/JakeRoggenbuck/kronicler/refs/heads/main/images/Kronicler-logo.svg)

Automatic performance capture and analytics for production applications in Python using a custom columnar database written in Rust.

[![Rust](https://img.shields.io/badge/Rust-1A5D8A?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/JakeRoggenbuck?tab=repositories&q=&type=&language=rust&sort=stargazers)
[![Python](https://img.shields.io/badge/Python-3776AB?style=for-the-badge&logo=python&logoColor=white)](https://github.com/JakeRoggenbuck?tab=repositories&q=&type=&language=python&sort=stargazers)
[![Rust](https://img.shields.io/github/actions/workflow/status/jakeroggenbuck/kronicler/CI.yml?branch=main&style=for-the-badge)](https://github.com/JakeRoggenbuck/kronicler/actions)
[![Version](https://img.shields.io/pypi/v/kronicler?style=for-the-badge)](https://pypi.org/project/kronicler)
[![Crates Version](https://img.shields.io/crates/v/kronicler?style=for-the-badge)](https://crates.io/crates/kronicler)
[![Downloads](https://img.shields.io/crates/d/kronicler?style=for-the-badge)](https://crates.io/crates/kronicler)

View Kronicler on [UseKronicler.com](https://usekronicler.com), [PyPi.org](https://pypi.org/project/kronicler), [Crates.io](https://crates.io/crates/kronicler) and [GitHub](https://github.com/JakeRoggenbuck/kronicler).

> [!IMPORTANT]
> Kronicler is still early in development. I'd appreciate any feedback to help make kronicler more useful for you!

## Benefits of using Kronicler

- Automatic performance capturing
- Lightweight and concurrent\*
- One Python dependency
- Works out-of-the-box without configuration 

\* concurrency is in development but not fully implemented as of version 0.1.2. Track concurrency in [issue #123](https://github.com/JakeRoggenbuck/kronicler/issues/123).

## Why use Kronicler?

If you want to monitor the performance of a production application, kronicler offers efficient and lightweight logging with a single library. Kronicler lets you view runtime statistics for functions like mean and median as well as statistics for different percentiles.

A use-case for these statistics is to find functions that occasionally operate much slower than they do on average. By looking at the "slowest" speed, the standard error, and the mean, you can find functions that occasionally run much slower than expected. Sometimes it's hard to find and replicate these issues in a test environment, so keeping logs in your production application can improve your ability to find these issues.

## What really is Kronicler?


| Name                     | Description                                                                                                                                        | Link                                                                                                                           |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| **Kronicler (Python)**   | A [Python library](https://pypi.org/project/kronicler/) that provides the `@kronicler.capture` decorator to save performance logs to a database.   | [More about Python](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#install-python)                             |
| **Kronicler (Database)** | Custom columnar database designed for log capturing and performance analytics.                                                                     | [More about Database](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#architecture)                             |
| **Kronicler (Rust)**     | A [Rust library](https://crates.io/crates/kronicler) for accessing the columnar database and capture functions for Rust-based log capturing tasks. | [More about Rust](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#using-kroniclers-database-directly)           |
| **Kronicler (CLI)**      | Rust-based CLI tool for analyzing performance logs captured with the capture methods.                                                              | [More about CLI](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analytics-cli)                                 |
| **Kronicler (Web)**      | Prototype web portal for remotely viewing performance logs.                                                                                        | [More about Web](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analytics-web-dashboard)                       |

## Install (Python)

#### Install with Pip for Python

```sh
pip install kronicler
```

You may need to use a [virtual environment](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#python-virtual-environment).

## Usage (Python)

Kronicler provides a Python decorator called `capture` that will calculate the time it takes to run the function.

```python
import kronicler

@kronicler.capture
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

For FastAPI, to capture each route, you can use the `KroniclerMiddleware` that will be included in `v0.1.2`. This allows you to capture every route that gets called.

```python
from fastapi import FastAPI
import uvicorn
import kronicler

app = FastAPI()
app.add_middleware(kronicler.KroniclerMiddleware)

# Used only for the /logs route
DB = kronicler.Database(sync_consume=True)


@app.get("/")
def read_root():
    return {"Hello": "World"}


@app.get("/logs")
def read_logs():
    return DB.logs()


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
```

If you want to capture functions manually, you can still do that with `@kronicler.capture` as normal. This is helpful if you want to benchmark functions that are not routes.

```python
from fastapi import FastAPI
import uvicorn
import kronicler

app = FastAPI()
app.add_middleware(kronicler.KroniclerMiddleware)

# Used only for the /logs route
DB = kronicler.Database(sync_consume=True)


# You need to wrap helper functions
@kronicler.capture
def foo():
    return {"Hello": "World"}


# You cannot wrap routes right now
@app.get("/")
def read_root():
    return foo()

# Return the logs to the user (optional)
@app.get("/logs")
def read_logs():
    return DB.logs()


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
```

Code from [tests/fastapi-test/main.py](https://github.com/JakeRoggenbuck/kronicler/blob/main/tests/fastapi-test/main.py).

## Using Kronicler manually

It's recommended to first use Kronicler's built-in `capture` decorator. However, if you want to write your own capture statements and fetch data yourself, you can use the functions in that come with the Python library.

```python
import kronicler

DB = kronicler.Database(sync_consume=True)

def foo():
    DB.capture("String Value", [], 100, 200)

# Call the function
foo()

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
kronicler = "0.1.2"
```

To get a good idea of how to use Kronicler's internal Rust database, I'd recommend looking at some of the tests in the Rust files. You can also look at the source code for the `kr` binary in [main.rs](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/bin/main.rs).

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

Kronicler is designed to be as lightweight as possible. By adding logs to a queue concurrently\*, Kronicler doesn't affect performance by much \[[PR #73](https://github.com/JakeRoggenbuck/kronicler/pull/73), [PR #76](https://github.com/JakeRoggenbuck/kronicler/pull/76)\].

For accessing logs and running calculations, Kronicler uses a columnar database design to optimize file operations when looking at lots of data from only a few columns typical of analytics tasks.

\* concurrency is in development but not fully implemented as of version 0.1.2. Track concurrency in [issue #123](https://github.com/JakeRoggenbuck/kronicler/issues/123).

### Kronicler DB vs. SQLite

I implemented a [test version of Kronicler](https://github.com/JakeRoggenbuck/kronicler/tree/main/tests/kronicler-sqlite-comparison) logging that uses SQLite under the hood to benchmark against.

#### Experiment 1. Insert 1000 captures

Here is the function that we capture with `kronicler_sqlite.capture` defined by our test package that uses SQLite.

```python
@kronicler_sqlite.capture
def foo_1():
    val = 9

    for x in range(4):
        val += val + x

    return val
```

Here is where we call the function `CAPTURE_COUNT` times. Which in this case is 1000.

```python
def test_sqlite():
    ## Test for kronicler_sqlite

    # Warmup
    for _ in range(WARMUP_COUNT):
        foo_1()

    # Test
    for _ in range(CAPTURE_COUNT):
        foo_1()
```

We do the same for the Kronicler DB version with `kronicler.capture`.

We then run each of those functions 50 times and log the results:

```python
if __name__ == "__main__":
    insert_times_data = []
    avg_times_data = []

    for x in tqdm.tqdm(range(REPEATS)):
        # TEST sqlite inserts
        start = time.time_ns()
        test_sqlite()
        end = time.time_ns()
        print(f"{test_sqlite.__name__} took {end - start}ns")
        insert_times_data.append((test_sqlite.__name__, end - start))

        # TEST sqlite avg
        start = time.time_ns()
        avg_val = avg_sqlite()
        end = time.time_ns()
        print(f"{avg_sqlite.__name__} = {avg_val} and took {end - start}ns")
        avg_times_data.append((avg_sqlite.__name__, end - start))

        # Wait for any cleanup to happen between SQLite and Columnar
        time.sleep(2)

        # TEST columnar inserts
        start = time.time_ns()
        test_columnar()
        end = time.time_ns()
        print(f"{test_columnar.__name__} took {end - start}ns")
        insert_times_data.append((test_columnar.__name__, end - start))

        # TEST columnar avg
        start = time.time_ns()
        avg_val = avg_columnar()
        end = time.time_ns()
        print(f"{avg_columnar.__name__} = {avg_val} and took {end - start}ns")
        avg_times_data.append((avg_columnar.__name__, end - start))

        # Wait for any cleanup to happen between Columnar and No log
        time.sleep(2)

        # TEST no log inserts
        start = time.time_ns()
        test_no_logging()
        end = time.time_ns()
        print(f"{test_no_logging.__name__} took {end - start}ns")
        insert_times_data.append((test_no_logging.__name__, end - start))

        # TEST no logging avg
        start = time.time_ns()
        avg_val = avg_no_logging()
        end = time.time_ns()
        print(f"{avg_no_logging.__name__} = {avg_val} and took {end - start}ns")
        avg_times_data.append((avg_no_logging.__name__, end - start))

    with open("sync_insert_data.json", "w") as file:
        json.dump(insert_times_data, file)

    with open("sync_avg_data.json", "w") as file:
        json.dump(avg_times_data, file)
```

This test is called [simple_sync.py](tests/kronicler-sqlite-comparison/tests/simple_sync.py). Note: concurrent mode is turned off as of v0.1.1, so those tests will not run. The concurrent version of the test is [simple_concurrent.py](tests/kronicler-sqlite-comparison/tests/simple_concurrent.py) and will exit with an error message explaining the lack of support in v0.1.1.

##### Insert (writing data to disk):

<img width="800" height="500" alt="sync_insert_50" src="https://github.com/user-attachments/assets/712ee44d-e879-4d2c-85fe-219f4d36b373" />

The data can be found at [sync_insert_data.json](tests/kronicler-sqlite-comparison/tests/sync_insert_data.json).

Inserting is **7.71x** faster than SQLite.

It's still unknown why SQLite has large latency spikes, but information about that will be mentioned in [issue #101](https://github.com/JakeRoggenbuck/kronicler/issues/101) if found.

##### Average (calculating the mean):

<img width="800" height="500" alt="sync_avg_50" src="https://github.com/user-attachments/assets/33e5fb2a-92e8-4807-a1eb-bb89291572b8" />

The data can be found at [sync_avg_data.json](tests/kronicler-sqlite-comparison/tests/sync_avg_data.json).

Kronicler uses amortized constant calculations to keep an average runtime for functions. This causes the mean calculation to be **837.24x** faster than the average function in SQLite.

Here is how it's done in SQLite:

```sql
SELECT AVG(delta) FROM function_calls WHERE function_name = ?
```

Kronicler keeps an updated mean on insert. This same approach can be done with SQLite by having your own index on function names and a table to keep track of the mean function time the same way Kronicler does it, however that's not the default and you'd have to build that into your implementation.

### Kronicler Mean with Large Data Test

#### Correctness for average (mean) calculation for large data

##### The test script

```sh
rm -rf .kronicler_data

sleep 2

python3 ./large_average.py

echo -e "Finished running large_average.\n\n"

rm -rf .kronicler_data

sleep 2

python3 ./large_average_no_in_mem_comp.py

echo -e "Finished running large_average_no_in_mem_comp.\n\n"

echo "Note: large_average has a higher insert time because it needs to store ground truth values in memory. The ground truth average will also be quicker because it uses only in-memory values."
```

This script is called [run_large_avg.sh](tests/python-integration-tests/run_large_avg.sh).

Importantly, we test that the average is correct using a "ground truth" calculation.

```py
start = time.time()
kr_avg = DB.average("jake")
kr_avg_time = time.time() - start
print("Kronicler average: ", kr_avg, f"ran in {kr_avg_time}.")

start = time.time()
py_avg = mean(ground_truth_data)
py_avg_time = time.time() - start
print("Ground truth average: ", py_avg, f"ran in {py_avg_time}.")

assert kr_avg - py_avg < 0.0001
```

This is from [large_average.py](tests/python-integration-tests/large_average.py).

##### Results

```
Running insert took 76.3846685886383
First row: Row(id=0, fields=["jake", Epoch(169), Epoch(367), Epoch(198)])
Fetched all 100000 rows.
Kronicler average:  200.19449000000037 ran in 2.1696090698242188e-05.
Ground truth average:  200.19449 ran in 0.0007288455963134766.
Finished running large_average.


Running insert took 77.20267271995544
First row: Row(id=0, fields=["jake", Epoch(191), Epoch(372), Epoch(181)])
Fetched all 100000 rows.
Kronicler average:  199.92816000000082 ran in 2.3126602172851562e-05.
Finished running large_average_no_in_mem_comp.
```

As you can see, in the first test the kronicler_data amortized const average 
took `2.1696090698242188e-05` seconds but the ground truth written in Python 
`0.0007288455963134766` seconds. The Rust amortized constant is faster.

## Analytics Web Dashboard

The Analytics Web Dashboard is still under construction. This feature will let you remotely view the logs collected from Kronicler.

<img width="1904" height="944" alt="image" src="https://github.com/user-attachments/assets/cc2b7b32-cbde-4a05-a630-a4cc4ab83444" />

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

By adding the `capture` decorator to your code (as seen below), Kronicler will automatically test the runtime of your function when it gets called. The results of the test get added to the database. This data can later be viewed in the [Analytics Web Dashboard](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analytics-web-dashboard) or the [Analytics CLI](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#analytics-cli).

```python
import kronicler

@kronicler.capture()
def my_function():
	pass
```

### Disable kronicler with env var

To temporarily turn off logging, you can set `KRONICLER_ENABLED=false`. The default for `KRONICLER_ENABLED` is `true`, so it does not need to be set to make logging work.

## Development

#### Building the Python package

##### 1. Build the package

```sh
maturin build
```

You will need maturin installed, see [Install Maturin](https://github.com/JakeRoggenbuck/kronicler?tab=readme-ov-file#install-maturin).

##### 2. Install the package

```sh
pip install --force-reinstall target/wheels/kronicler-*
```

#### Testing Python scripts

You can run the scripts in [tests/](https://github.com/JakeRoggenbuck/kronicler/tree/main/tests) to test out the functionality of Kronicler.

```sh
python3 tests/many_test.py
```

#### Testing Rust

You can run the testing suite with the following command:

```
cargo t
```

We use `cargo t` as an alias to run the tests in one thread.

The tests should all succeed

<img width="1309" height="1006" alt="image" src="https://github.com/user-attachments/assets/474f58d5-0195-439d-9218-99735de8862d" />

## Publishing

#### Publishing to Crates.io

##### 1. Increment the version everywhere

##### 2. Run the publish dry-run

```
cargo publish --dry-run
```

##### 3. Check package list

```
cargo package --list
```

Check that only the needed Rust files are included.

##### 4. Publish!

```
cargo publish
```

#### Publishing to PyPi.org

Use maturin to publish the package

```
maturin publish --username __token__ --password <API Token>
```

You may need to get a new API token from [PyPi.org](https://pypi.org/manage/account/token/).

## Misc Docs

#### Python Virtual Environment

Create the virtual environment

```py
python3 -m venv .venv
```

Enter the virtual environment

```
source .venv/bin/activate
```

I usually call my virtual environment either `.venv` or more frequently just `venv`

#### Install Maturin

```sh
pip install maturin
```

For more info, see the [maturin docs](https://github.com/PyO3/maturin).

#### Rust Logging

Kronicler uses [env_logger](https://docs.rs/env_logger/latest/env_logger/) to log internals. To view these logs, you add the `RUST_LOG` env var.

For instance, you can include the logger for `cargo run` with the `fetch` argument.

```
RUST_LOG=info cargo run -- --fetch 0
```

This will print the logs:

<img width="1171" height="653" alt="image" src="https://github.com/user-attachments/assets/60b7cab1-95f5-4f03-a73d-43660db6e5af" />

##### Adding a log

You can add logs with the `info!` macro. There are also `debug!`, `warn!`, `trace!`, and `error!` variants.

```rs
use log::info;

fn main() {
    let _ = env_logger::try_init();

    info!("Some info here");
}
```

You only need to call `try_init` once, and that's already done in [lib.rs](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/lib.rs) and [main.rs](https://github.com/JakeRoggenbuck/kronicler/blob/main/src/bin/main.rs).

#### Formatting for Python

Kronicler uses [Ruff](https://github.com/astral-sh/ruff) for formatting.

You can format all Python files with:

```
ruff format *.py
```

You can also check that they have the right format with:

```
ruff check *.py
```

#### Formatting for Rust

Kronicler uses `cargo fmt`

<!-- :frog: -->
