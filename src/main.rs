use rusty_pomodoro::cli::CLI;
use rusty_pomodoro::record::Record;
use rusty_pomodoro::files::*;
use rusty_pomodoro::pomodoro::Pomodoro;

fn main() {
    let _csv_file = CsvFile::new("pom-record.csv".to_string());
    let _record = Record::new(&_csv_file);
    let mut _pomodoro: Pomodoro;
    match _record.no_of_finished_pomodoros_from_record() {
        Some(no) => _pomodoro = Pomodoro::continue_from(no),
        None => _pomodoro = Pomodoro::new(), 
    }
    let _cli = CLI::new();
    _record.initialize();
    _pomodoro.add_observer(&_record);
    _pomodoro.add_observer(&_cli);
    _pomodoro.proceed();
}
