use crate::files::recordfile::RecordFile;
use std::error::Error;

pub struct NullFile {
    trigger_success: bool,
    date: String,
}

impl NullFile {
    pub fn new(trigger_success: bool, date: String) -> NullFile {
        NullFile {
            trigger_success: trigger_success,
            date: date,
        }
    }
}

impl RecordFile for NullFile {
    fn open_or_create_with_headers(&self, headers: &Vec<String>) {
        //
    }
    fn write_headers(&self, headers: &Vec<String>) -> Result<(), Box<Error>> {
        Ok(())
    }
    fn write_record_to_new_line(&self, record: Vec<String>) -> Result<(), Box<Error>> {
        Ok(())
    }
    fn overwrite_record_in_pos_with(&self, pos: usize, record: Vec<String>) -> Result<(), Box<Error>> {
        Ok(())
    }
    fn get_last_pomodoro_count(&self) -> Option<u32> {
        if self.trigger_success {
            Some(10)
        } else {
            None
        }
    }
    fn get_last_pomodoro_date_and_line_no(&self) -> Option<(String, usize)> {
        if self.trigger_success {
            Some((self.date.clone(), 1))
        } else {
            None
        }
    }
}
