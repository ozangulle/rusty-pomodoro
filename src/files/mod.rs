mod csvfile;
mod recordfile;

pub use csvfile::CsvFile;
pub use recordfile::RecordFile;

#[cfg(test)]
pub mod nullfile;
