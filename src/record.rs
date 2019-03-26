use crate::Observer;
use crate::Pomodoro;
use std::io::{BufRead, BufReader, Write};
use std::fs::{File,OpenOptions};
use std::error::Error;
use chrono::prelude::*;

pub struct Record {
    filename: String,
    current_date: String,
}

impl Record {
    pub fn new(filename: &str) -> Record {
        Record {
            filename: filename.to_string(),
            current_date: Utc::now().format("%Y-%m-%d").to_string(),
        }
    }

    pub fn initialize(&self) {
        self.check_record();
    }

    pub fn no_of_finished_pomodoros_from_record(&self) -> Option<u32> {
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

    fn check_record(&self) {
        let record = File::open(&self.filename);
        match record {
            Ok(_) => println!("Record file found"),
            Err(_) => {
                let created = self.create_record_file();
                match created {
                    Ok(_) => println!("{} {}", "Created record file:", &self.filename),
                    Err(_) => println!("Could not create the record file."),
                }
            },
        }
    }

    fn create_record_file(&self) -> Result<(), Box<Error>> {
        self.write_to_record_file(&"Date",&"Number of Pomodoros")?;
        Ok(())
    }

    fn process(&self, _p: &Pomodoro) {
        match self.write_record(_p) {
            Ok(()) => (),
            Err(_) => println!("Error: There was an error while writing to the record.")
        }
    }

    fn write_record(&self, p: &Pomodoro) -> Result<(), Box<Error>> {
        match self.last_entry_in_record() {
            Some(last_line) => {
                self.overwrite_record(last_line, &p.finished_pomodoros);
                Ok(())
            },
            None => {
                self.write_to_record_file(&self.current_date, &p.finished_pomodoros())?;
                Ok(())
            }
        }
    }

    fn last_entry_in_record(&self) -> Option<usize> {
        let file = File::open(&self.filename);
        match file {
            Ok(file) => {
                let mut line_position: usize = 0;
                for line in BufReader::new(file).lines() {
                    if line.is_ok() {
                        line_position += 1;
                    }
                }
                if line_position > 1 {
                    return Some(line_position);
                }
                return None;
            },
            Err(_) => return None,
        }
    }

    fn overwrite_record(&self, position: usize, no_of_pomodoros: &u32) {
        let mut record_file = Vec::new();
        let read_file = File::open(&self.filename).expect("Could not open file.");
        let reader = BufReader::new(read_file);
        for line in reader.lines() {
            let line = line.unwrap();
            record_file.push(line);
        }
        record_file[position - 1] = format!("{},{}", &self.current_date, no_of_pomodoros.to_string());
        let mut write_file = File::create(&self.filename).expect("Could not open file.");
        for entry in &record_file {                                                                                                                                                                  
            writeln!(write_file, "{}", entry).expect("Could not write entry to file.");                                                                                                                            
        }  
    }

    fn write_to_record_file(&self, date: &str, no: &str) -> Result<(), Box<Error>> {
        let mut write_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&self.filename)?;
        writeln!(write_file, "{},{}", date, no)?;
        Ok(())
    }
}

impl Observer for Record {
    fn callback(&self, p: &Pomodoro) {
        if p.finished_pomodoros > 0 {
            self.process(p);
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate remove_dir_all;
    use remove_dir_all::*;
    use std::fs::{File, DirBuilder};
    use std::io::{BufRead, BufReader};
    use crate::record::Record;
    use crate::pomodoro::*;
    use crate::observer::Observer;
    use std::thread;
    use std::time::Duration;
    use serial_test_derive::serial;

    static record_name: &str = "./temp/record.csv";
    
    fn setup() {
        DirBuilder::new().create("./temp").unwrap();
    }

    fn clean_up() {
        remove_dir_all("./temp").unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    fn get_last_entry_and_line_no() -> Option<(String, u8)> {
        let file = File::open(record_name);
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
    fn creates_a_record_file_if_none_exists() {
        setup();
        let mut record = Record::new(record_name);
        record.initialize();
        let file = File::open(record_name.to_string());
        assert!(file.is_ok());
        clean_up();
    }

    #[test]
    #[serial]
    fn calling_callback_with_pomodoro_records_the_state() {
        setup();
        let record = Record::new(record_name);
        record.initialize();
        let pom = Pomodoro::continue_from(10);
        record.callback(&pom);
        match get_last_entry_and_line_no() {
            Some((last_line, line_no)) => {
                assert!(line_no == 2);
                clean_up();
            },
            None => {
                panic!("Something went wrong");
                clean_up();
            },
        }
    }
}