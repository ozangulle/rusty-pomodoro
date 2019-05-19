use crate::files::RecordFile;
use crate::observers::Observer;
use crate::pomodoro_core::PomodoroStates;
use chrono::prelude::*;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Record {
    record_file: Arc<Mutex<dyn RecordFile>>,
}

impl Record {
    pub fn new(record_file: Arc<Mutex<dyn RecordFile>>) -> Record {
        Record { record_file }
    }

    pub fn initialize(&self) {
        let headers =
            self.construct_content_vec("Date".to_string(), "Number of pomodoros".to_string());
        self.record_file
            .lock()
            .unwrap()
            .open_or_create_with_headers(&headers);
    }

    pub fn no_of_finished_pomodoros_from_record(&self) -> Option<u32> {
        let locked_file = self.record_file.lock().unwrap();
        match locked_file.get_last_pomodoro_date_and_line_no() {
            Some((last_date, _line_no)) => {
                if last_date == self.get_current_date() {
                    locked_file.get_last_pomodoro_count()
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn process(&self, _next_state: PomodoroStates, finished_pomodoros: u32) {
        match self.write_record(finished_pomodoros) {
            Ok(()) => (),
            Err(_) => println!("Error: There was an error while writing to the record."),
        }
    }

    fn write_record(&self, finished_pomodoros: u32) -> Result<(), Box<Error>> {
        let content_vec =
            self.construct_content_vec(self.get_current_date(), finished_pomodoros.to_string());
        match self
            .record_file
            .lock()
            .unwrap()
            .get_last_pomodoro_date_and_line_no()
        {
            Some((last_date, line_pos)) => {
                let record_file: Arc<Mutex<RecordFile>> = self.record_file.clone();
                if last_date == self.get_current_date() {
                    thread::spawn(move || {
                        record_file
                            .lock()
                            .unwrap()
                            .overwrite_record_in_pos_with(line_pos, content_vec)
                            .expect("error");
                    });
                } else {
                    thread::spawn(move || {
                        record_file
                            .lock()
                            .unwrap()
                            .write_record_to_new_line(content_vec)
                            .expect("error");
                    });
                }
            }
            None => {
                let record_file: Arc<Mutex<RecordFile>> = self.record_file.clone();
                thread::spawn(move || {
                    record_file
                        .lock()
                        .unwrap()
                        .write_record_to_new_line(content_vec)
                        .expect("error");
                });
            }
        }
        Ok(())
    }

    fn construct_content_vec(&self, first_str: String, sec_str: String) -> Vec<String> {
        let mut vec = Vec::new();
        vec.push(first_str.clone());
        vec.push(sec_str.clone());
        vec
    }

    fn get_current_date(&self) -> String {
        Utc::now().format("%Y-%m-%d").to_string()
    }
}

impl Observer for Record {
    fn callback(&self, next_state: PomodoroStates, finished_pomodoros: u32) {
        if finished_pomodoros > 0 {
            self.process(next_state, finished_pomodoros);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::files::nullfile::NullFile;
    use crate::record::Record;
    use chrono::prelude::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_last_pomodoro_not_from_today() {
        let null_file = NullFile::new(true, String::from("1970-01-01"));
        let record = Record::new(Arc::new(Mutex::new(null_file)));
        assert_eq!(record.no_of_finished_pomodoros_from_record(), None);
    }

    #[test]
    fn test_last_pomodoro_from_today() {
        let null_file = NullFile::new(true, Utc::now().format("%Y-%m-%d").to_string());
        let record = Record::new(Arc::new(Mutex::new(null_file)));
        assert_eq!(record.no_of_finished_pomodoros_from_record(), Some(10));
    }
}
