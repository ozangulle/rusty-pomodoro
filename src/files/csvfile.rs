use crate::files::recordfile::RecordFile;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct CsvFile {
    filename: String,
}

impl CsvFile {
    pub fn new(filepath: String, filename: String) -> CsvFile {
        CsvFile {
            filename: filepath.to_string() + "/" + &filename + ".csv",
        }
    }

    fn append_new_line(&self, contents: &[String]) -> Result<(), Box<Error>> {
        let mut write_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.filename)?;
        let write_string = self.create_csv_line_from_vec(contents);
        writeln!(write_file, "{}", write_string)?;
        Ok(())
    }

    fn create_csv_line_from_vec(&self, vec: &[String]) -> String {
        let mut write_string = String::new();
        for (i, content) in vec.iter().enumerate() {
            if i < &vec.len() - 1 {
                write_string.push_str(content);
                write_string.push_str(",");
            } else {
                write_string.push_str(content);
            }
        }
        write_string
    }
}

impl RecordFile for CsvFile {
    fn open_or_create_with_headers(&self, headers: &[String]) {
        let record = File::open(&self.filename);
        match record {
            Ok(_) => println!("Record file found"),
            Err(_) => {
                let created = File::create(&self.filename);
                self.write_headers(headers)
                    .expect("Headers could not be written");
                match created {
                    Ok(_) => println!("Created record file: {}", &self.filename),
                    Err(_) => println!("Could not create the record file."),
                }
            }
        }
    }

    fn write_headers(&self, headers: &[String]) -> Result<(), Box<Error>> {
        self.append_new_line(headers)
    }

    fn write_record_to_new_line(&self, record: Vec<String>) -> Result<(), Box<Error>> {
        self.append_new_line(&record)
    }

    fn overwrite_record_in_pos_with(
        &self,
        pos: usize,
        record: Vec<String>,
    ) -> Result<(), Box<Error>> {
        let mut record_file = Vec::new();
        let read_file = File::open(&self.filename).expect("Could not open file.");
        let reader = BufReader::new(read_file);
        for line in reader.lines() {
            let line = line.unwrap();
            record_file.push(line);
        }
        let write_string = self.create_csv_line_from_vec(&record);
        record_file[pos - 1] = write_string.to_string();
        let mut write_file = File::create(&self.filename).expect("Could not open file.");
        for entry in &record_file {
            writeln!(write_file, "{}", entry)?;
        }
        Ok(())
    }

    fn get_last_pomodoro_count(&self) -> Option<u32> {
        let file = File::open(&self.filename);
        match file {
            Ok(file) => {
                let mut line_position: usize = 0;
                let mut last_line = String::new();
                for line in BufReader::new(file).lines() {
                    if line.is_ok() {
                        last_line = line.unwrap();
                        line_position += 1;
                    }
                }
                if line_position > 1 {
                    let split_line: Vec<&str> = last_line.split(',').collect();
                    let finished_pomodoros_string = split_line.last().unwrap();
                    let finished_pomodoros_int = finished_pomodoros_string.parse::<u32>().unwrap();
                    return Some(finished_pomodoros_int);
                }
                None
            }
            Err(_) => None,
        }
    }

    fn get_last_pomodoro_date_and_line_no(&self) -> Option<(String, usize)> {
        let file = File::open(&self.filename);
        match file {
            Ok(file) => {
                let mut line_position: usize = 0;
                let mut last_line = String::new();
                for line in BufReader::new(file).lines() {
                    if line.is_ok() {
                        last_line = line.unwrap();
                        line_position += 1;
                    }
                }
                if line_position > 1 {
                    let split_line: Vec<&str> = last_line.split(',').collect();
                    let last_pomodoro_date = split_line.first().unwrap().to_string();
                    return Some((last_pomodoro_date, line_position));
                }
                None
            }
            Err(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate remove_dir_all;
    use crate::files::csvfile::CsvFile;
    use crate::files::recordfile::RecordFile;
    use remove_dir_all::*;
    use serial_test_derive::serial;
    use std::fs::{DirBuilder, File};
    use std::io::{BufRead, BufReader};
    use std::thread;
    use std::time::Duration;

    static FILEPATH: &str = "./temp";
    static FILENAME: &str = "record";
    static FILEPATH_AND_NAME_WITH_SUFFIX: &str = "./temp/record.csv";

    fn setup() {
        DirBuilder::new().create("./temp").unwrap();
    }

    fn clean_up() {
        remove_dir_all("./temp").unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    fn get_last_entry_and_line_no() -> Option<(String, u8)> {
        let file = File::open(FILEPATH_AND_NAME_WITH_SUFFIX);
        match file {
            Ok(file) => {
                let mut line_position: u8 = 0;
                let mut last_line: String = String::new();
                for line in BufReader::new(file).lines() {
                    if line.is_ok() {
                        last_line = line.unwrap();
                        line_position += 1;
                    }
                }
                if line_position > 0 {
                    return Some((last_line.clone(), line_position));
                }
                return None;
            }
            Err(_) => return None,
        }
    }

    fn header_vec() -> Vec<String> {
        vec![String::from("Test"), String::from("Headers")]
    }

    fn content_vec(first: &str, sec: &str) -> Vec<String> {
        vec![String::from(first), String::from(sec)]
    }

    #[test]
    #[serial]
    fn creates_a_file_if_none_exists() {
        setup();
        let file = CsvFile::new(FILEPATH.to_string(), FILENAME.to_string());
        let headers = header_vec();
        file.open_or_create_with_headers(&headers);
        let raw_file = File::open(FILEPATH_AND_NAME_WITH_SUFFIX);
        assert!(raw_file.is_ok());
        clean_up();
    }

    #[test]
    #[serial]
    fn creating_headers() {
        setup();
        let file = CsvFile::new(FILEPATH.to_string(), FILENAME.to_string());
        let headers = header_vec();
        file.write_headers(&headers).expect("Something went wrong");
        match get_last_entry_and_line_no() {
            Some((last_entry, line_no)) => {
                clean_up();
                assert!(line_no == 1);
                assert_eq!(last_entry, format!("{},{}", "Test", "Headers"));
            }
            None => {
                clean_up();
                panic!();
            }
        }
    }

    #[test]
    #[serial]
    fn appending_lines() {
        setup();
        let file = CsvFile::new(FILEPATH.to_string(), FILENAME.to_string());
        let headers = header_vec();
        file.write_headers(&headers).expect("Something went wrong");
        let content = content_vec("2019-01-01", "1");
        file.append_new_line(&content)
            .expect("Something went wrong");
        match get_last_entry_and_line_no() {
            Some((last_entry, line_no)) => {
                clean_up();
                assert!(line_no == 2);
                assert_eq!(last_entry, format!("{},{}", "2019-01-01", "1"));
            }
            None => {
                clean_up();
                panic!();
            }
        }
    }

    #[test]
    #[serial]
    fn overwrite_last_line() {
        setup();
        let file = CsvFile::new(FILEPATH.to_string(), FILENAME.to_string());
        let headers = header_vec();
        file.write_headers(&headers).expect("Something went wrong");
        let mut content = content_vec("2019-01-01", "1");
        file.append_new_line(&content)
            .expect("Something went wrong");
        let pos: usize = 2;
        content = content_vec("2019-01-01", "2");
        file.overwrite_record_in_pos_with(pos, content)
            .expect("Something went wrong");
        match get_last_entry_and_line_no() {
            Some((last_entry, line_no)) => {
                clean_up();
                assert!(line_no == 2);
                assert_eq!(last_entry, format!("{},{}", "2019-01-01", "2"));
            }
            None => {
                clean_up();
                panic!();
            }
        }
    }

    #[test]
    #[serial]
    fn get_last_pom_count() {
        setup();
        let file = CsvFile::new(FILEPATH.to_string(), FILENAME.to_string());
        let headers = header_vec();
        file.write_headers(&headers).expect("Something went wrong");
        let mut content = content_vec("2019-01-01", "1");
        file.append_new_line(&content)
            .expect("Something went wrong");
        let pos: usize = 2;
        content = content_vec("2019-01-01", "2");
        file.overwrite_record_in_pos_with(pos, content)
            .expect("Something went wrong");
        match file.get_last_pomodoro_count() {
            Some(no) => {
                clean_up();
                assert!(no == 2);
            }
            None => {
                clean_up();
                panic!();
            }
        }
    }

    #[test]
    #[serial]
    fn test_get_last_pomodoro_date_and_line_no() {
        setup();
        let file = CsvFile::new(FILEPATH.to_string(), FILENAME.to_string());
        let headers = header_vec();
        file.write_headers(&headers).expect("Something went wrong");
        let mut content = content_vec("2019-01-01", "1");
        file.append_new_line(&content)
            .expect("Something went wrong");
        let pos: usize = 2;
        content = content_vec("2019-01-01", "2");
        file.overwrite_record_in_pos_with(pos, content)
            .expect("Something went wrong");
        match file.get_last_pomodoro_date_and_line_no() {
            Some((last_date, line_no)) => {
                clean_up();
                assert!(line_no == 2);
                assert_eq!(last_date, "2019-01-01");
            }
            None => {
                clean_up();
                panic!();
            }
        }
    }
}
