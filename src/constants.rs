// How many logs need to be in the queue before we write to the database
// Make configurable: https://github.com/JakeRoggenbuck/kronicler/issues/18
pub const DB_WRITE_BUFFER_SIZE: usize = 0;

pub const DATA_DIRECTORY: &str = ".kronicler_data";

pub const PAGE_SIZE: usize = 4096;

pub const CONSUMER_DELAY: u64 = 1; // Half a second
