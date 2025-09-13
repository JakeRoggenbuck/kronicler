use kronicler::database::Database;
use log::debug;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
enum Fetch {
    All,
    One(usize),
}

impl FromStr for Fetch {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.to_lowercase();

        if s_lower == "all" {
            return Ok(Fetch::All);
        }

        if let Ok(num) = s.parse::<usize>() {
            return Ok(Fetch::One(num));
        }

        Err(format!("Unknown fetch option: {}", s))
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "kronicler")]
struct Opt {
    #[structopt(short, long)]
    fetch: Fetch,
}

/// Setup env logging
///
/// To use the logger, import the debug, error, or info macro from the log crate
///
/// Then you can add the macros to code like debug!("Start database!");
/// When you go to run the code, you can set the env var RUST_LOG=debug
/// Docs: https://docs.rs/env_logger/latest/env_logger/
#[inline]
fn init_logging() {
    let _ = env_logger::try_init();
}

fn fetch_all() {
    let mut db = Database::new_reader(true);

    for row in db.fetch_all() {
        println!("{}", row.to_string());
    }
}

fn fetch_one(index: usize) {
    let mut db = Database::new_reader(true);

    let row = db.fetch(index);

    if let Some(r) = row {
        println!("{}", r.to_string());
    }
}

fn main() {
    init_logging();

    let opt = Opt::from_args();

    debug!("Passed args and logging");

    match opt.fetch {
        Fetch::All => {
            fetch_all();
        }
        Fetch::One(i) => {
            fetch_one(i);
        }
    }
}
