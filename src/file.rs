use std::io::{BufRead, BufReader, Write};
use std::fs::{File,OpenOptions};
use std::error::Error;

pub struct CsvFile {
    filename: String,
}

impl CsvFile {
    pub fn new(filename: String) -> CsvFile {
        CsvFile {
            filename: filename.to_string(),
        }
    }

    pub fn open_or_create(&self) {
        let record = File::open(&self.filename);
        match record {
            Ok(_) => println!("Record file found"),
            Err(_) => {
                let created = File::create(&self.filename);
                match created {
                    Ok(_) => println!("{} {}", "Created record file:", &self.filename),
                    Err(_) => println!("Could not create the record file."),
                }
            },
        }
    }

    pub fn write_headers(&self, headers: &Vec<&str>) -> Result<(), Box<Error>> {
        self.append_new_line(headers)
    }

    pub fn write_record_to_new_line(&self, record: &Vec<&str>) -> Result<(), Box<Error>> {
        self.append_new_line(record)
    }

    pub fn overwrite_record_in_pos_with(&self, pos: &usize, record: &Vec<&str>) {
        let mut record_file = Vec::new();
        let read_file = File::open(&self.filename).expect("Could not open file.");
        let reader = BufReader::new(read_file);
        for line in reader.lines() {
            let line = line.unwrap();
            record_file.push(line);
        }
        let write_string = self.create_csv_line_from_vec(record);
        record_file[pos - 1] = format!("{}", write_string);
        let mut write_file = File::create(&self.filename).expect("Could not open file.");
        for entry in &record_file {                                                                                                                                                                  
            writeln!(write_file, "{}", entry).expect("Could not write entry to file.");                                                                                                                            
        }  
    }

    pub fn get_last_pomodoro_count(&self) -> Option<u32> {
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
                    let split_line: Vec<&str> = last_line.split(",").collect();
                    let finished_pomodoros_string = split_line.last().unwrap();
                    let finished_pomodoros_int = finished_pomodoros_string.parse::<u32>().unwrap();
                    return Some(finished_pomodoros_int);
                }
                return None;
            },
            Err(_) => return None,
        }
    }

    fn append_new_line(&self, contents: &Vec<&str>) -> Result<(), Box<Error>> {
        let mut write_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.filename)?;
        let write_string = self.create_csv_line_from_vec(contents);
        writeln!(write_file, "{}", write_string)?;
        Ok(())
    }

    fn create_csv_line_from_vec(&self, vec: &Vec<&str>) -> String {
        let mut write_string = String::new();
        for (i, content) in vec.iter().enumerate() {
            if i < &vec.len() - 1 {
                write_string.push_str(*content);
                write_string.push_str(",");
            } else {
                write_string.push_str(*content);
            }
        }
        write_string
    }
}

#[cfg(test)]
mod tests {
    extern crate remove_dir_all;
    use remove_dir_all::*;
    use std::fs::{File, DirBuilder};
    use std::io::{BufRead, BufReader};
    use crate::record::Record;
    use crate::file::CsvFile;
    use crate::pomodoro::*;
    use crate::observer::Observer;
    use std::thread;
    use std::time::Duration;
    use serial_test_derive::serial;

    static filename: &str = "./temp/record.csv";
    
    fn setup() {
        DirBuilder::new().create("./temp").unwrap();
    }

    fn clean_up() {
        remove_dir_all("./temp").unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    fn get_last_entry_and_line_no() -> Option<(String, u8)> {
        let file = File::open(filename);
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
            },
            Err(_) => return None,
        }
    }

    #[test]
    #[serial]
    fn creates_a_file_if_none_exists() {
        setup();
        let file = CsvFile::new(filename.to_string());
        file.open_or_create();
        let raw_file = File::open(filename);
        assert!(raw_file.is_ok());
        clean_up();
    }

    #[test]
    #[serial]
    fn creating_headers() {
        setup();
        let file = CsvFile::new(filename.to_string());
        let mut headers = Vec::new();
        headers.push("Test");
        headers.push("Headers");
        file.write_headers(&headers);
        match get_last_entry_and_line_no() {
            Some((last_entry, line_no)) => {
                clean_up();
                assert!(line_no == 1);
                assert_eq!(last_entry, format!("{},{}", "Test", "Headers"));
            },
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
        let file = CsvFile::new(filename.to_string());
        let mut headers = Vec::new();
        headers.push("Test");
        headers.push("Headers");
        file.write_headers(&headers);
        let mut content = Vec::new();
        content.push("2019-01-01");
        content.push("1");
        file.append_new_line(&content);
        match get_last_entry_and_line_no() {
            Some((last_entry, line_no)) => {
                clean_up();
                assert!(line_no == 2);
                assert_eq!(last_entry, format!("{},{}", "2019-01-01", "1"));
            },
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
        let file = CsvFile::new(filename.to_string());
        let mut headers = Vec::new();
        headers.push("Test");
        headers.push("Headers");
        file.write_headers(&headers);
        let mut content = Vec::new();
        content.push("2019-01-01");
        content.push("1");
        file.append_new_line(&content);
        let pos: usize = 2;
        let mut content = Vec::new();
        content.push("2019-01-01");
        content.push("2");
        file.overwrite_record_in_pos_with(&pos, &content);
        match get_last_entry_and_line_no() {
            Some((last_entry, line_no)) => {
                clean_up();
                assert!(line_no == 2);
                assert_eq!(last_entry, format!("{},{}", "2019-01-01", "2"));
            },
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
        let file = CsvFile::new(filename.to_string());
        let mut headers = Vec::new();
        headers.push("Test");
        headers.push("Headers");
        file.write_headers(&headers);
        let mut content = Vec::new();
        content.push("2019-01-01");
        content.push("1");
        file.append_new_line(&content);
        let pos: usize = 2;
        let mut content = Vec::new();
        content.push("2019-01-01");
        content.push("2");
        file.overwrite_record_in_pos_with(&pos, &content);
        match file.get_last_pomodoro_count() {
            Some(no) => {
                clean_up();
                assert!(no == 2);
            },
            None => {
                clean_up();
                panic!();
            }
        }
    }
}