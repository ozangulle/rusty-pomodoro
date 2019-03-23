use crate::Observer;
use crate::Pomodoro;
use std::io::{BufRead, BufReader, Write};
use std::fs::{File, OpenOptions};
use std::error::Error;
use csv::{Writer};
use chrono::prelude::*;

pub struct Record {
    filename: String,
    current_date: String,
}

impl Record {
    pub fn new() -> Record {
        Record {
            filename: "pom-record.csv".to_string(),
            current_date: Utc::now().format("%Y-%m-%d").to_string(),
        }
    }

    pub fn initialize(&self) {
        self.check_record();
    }

    fn check_record(&self) {
        let record = File::open(&self.filename);
        match record {
            Ok(_) => println!("Record file found"),
            Err(_) => {
                let created = self.create_record_file();
                match created {
                    Ok(_) => println!("{} {}", "Created record file", &self.filename),
                    Err(_) => println!("Could not create the record file."),
                }
            },
        }
    }

    fn create_record_file(&self) -> Result<(), Box<Error>> {
        let mut wtr = Writer::from_path(&self.filename)?;
        wtr.write_record(&["Date", "Pomodoros"])?;
        wtr.flush()?;
        Ok(())
    }

    fn process(&self, _p: &Pomodoro) {
        self.write_record(_p);
    }

    fn write_record(&self, p: &Pomodoro) -> Result<(), Box<Error>> {
        match self.last_entry_in_record() {
            Some(last_line) => {
                self.overwrite_record(last_line, &p.finished_pomodoros);
                Ok(())
            },
            None => {
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&self.filename)
                .unwrap();
                let mut wtr = Writer::from_writer(file);
                wtr.write_record(&[&self.current_date, &p.finished_pomodoros()]);
                wtr.flush()?;
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

    fn overwrite_record(&self, position: usize, no_of_pomodoros: &u32) {
        let mut record_file = Vec::new();
        let read_file = File::open(&self.filename).expect("Could not open file.");
        let reader = BufReader::new(read_file);
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            record_file.push(line);
        }
        record_file[position - 1] = format!("{},{}", &self.current_date, no_of_pomodoros.to_string());
        let mut write_file = File::create(&self.filename).expect("Could not open file.");
        for entry in &record_file {                                                                                                                                                                  
            write!(write_file, "{}\n", entry);                                                                                                                            
        }  
    }
}

impl Observer for Record {
    fn callback(&self, p: &Pomodoro) {
        if p.finished_pomodoros > 0 {
            self.process(p);
        }
    }
}
