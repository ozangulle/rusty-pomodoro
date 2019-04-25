use rusty_pomodoro::cli::CLI;
use rusty_pomodoro::record::Record;
use rusty_pomodoro::files::*;
use rusty_pomodoro::pomodoro::Pomodoro;
use rusty_pomodoro::pomodoro::PomodoroConfig;
use std::sync::{Arc, Mutex};

fn main() {
    let config = PomodoroConfig {
        pomodoro_time_in_secs: 25 * 60,
        short_break_time_in_secs: 5 * 60,
        long_break_time_in_secs: 15 * 60,
        max_pomodoros: 4,
    };
    let _csv_file = Arc::new(Mutex::new(CsvFile::new("pom-record.csv".to_string())));
    let _record = Record::new(_csv_file);
    let mut _pomodoro: Pomodoro;
    match _record.no_of_finished_pomodoros_from_record() {
        Some(no) => _pomodoro = Pomodoro::continue_from(no, config),
        None => _pomodoro = Pomodoro::new(config), 
    }
    let _cli = CLI::new();
    _record.initialize();
    _pomodoro.add_observer(&_record);
    _pomodoro.add_observer(&_cli);
    _pomodoro.add_update_observer(&_cli);
    _pomodoro.proceed();
}
