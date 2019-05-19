use std::error::Error;

pub trait RecordFile: Send {
    fn open_or_create_with_headers(&self, headers: &[String]);
    fn write_headers(&self, headers: &[String]) -> Result<(), Box<Error>>;
    fn write_record_to_new_line(&self, record: Vec<String>) -> Result<(), Box<Error>>;
    fn overwrite_record_in_pos_with(
        &self,
        pos: usize,
        record: Vec<String>,
    ) -> Result<(), Box<Error>>;
    fn get_last_pomodoro_count(&self) -> Option<u32>;
    fn get_last_pomodoro_date_and_line_no(&self) -> Option<(String, usize)>;
}
