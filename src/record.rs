use crate::Observer;
use crate::Pomodoro;
use crate::CsvFile;
use chrono::prelude::*;
use std::error::Error;

pub struct Record<'a> {
    record_file: &'a CsvFile,
    current_date: String,
}

impl<'a> Record<'a> {
    pub fn new(record_file: &'a CsvFile) -> Record<'a> {
        Record {
            record_file,
            current_date: Utc::now().format("%Y-%m-%d").to_string(),
        }
    }

    pub fn initialize(&self) {
        let headers = self.construct_content_vec(
            &"Date".to_string(),
            &"Number of pomodoros".to_string(),
        );
        self.record_file.open_or_create_with_headers(&headers);
    }

    pub fn no_of_finished_pomodoros_from_record(&self) -> Option<u32> {
        self.record_file.get_last_pomodoro_count()
    }
    
    fn process(&self, _p: &Pomodoro) {
        match self.write_record(_p) {
            Ok(()) => (),
            Err(_) => println!("Error: There was an error while writing to the record.")
        }
    }

    fn write_record(&self, p: &Pomodoro) -> Result<(), Box<Error>> {
        let content_vec = self.construct_content_vec(&self.current_date, &p.finished_pomodoros.to_string());
        match self.record_file.get_last_pomodoro_date_and_line_no() {
            Some((last_date, line_pos)) => {
                if last_date == self.current_date {
                    self.record_file.overwrite_record_in_pos_with(&line_pos, &content_vec)?
                } else {
                    self.record_file.write_record_to_new_line(&content_vec)?
                }
            },
            None => self.record_file.write_record_to_new_line(&content_vec)?
        }
        Ok(())
    }

    fn construct_content_vec(&self, first_str: &String, sec_str: &String) -> Vec<String> {
        let mut vec = Vec::new();
        vec.push(first_str.clone());
        vec.push(sec_str.clone());
        return vec;
    }
}

impl<'a> Observer for Record<'a> {
    fn callback(&self, p: &Pomodoro) {
        if p.finished_pomodoros > 0 {
            self.process(p);
        }
    }
}
