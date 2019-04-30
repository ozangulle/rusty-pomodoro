use rusty_pomodoro::cli::CLI;
use rusty_pomodoro::communication::{ConcReceiver, ConcSender};
use rusty_pomodoro::files::*;
use rusty_pomodoro::pomodoro::Pomodoro;
use rusty_pomodoro::pomodoro::PomodoroConfig;
use rusty_pomodoro::record::Record;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let config = PomodoroConfig {
        pomodoro_time_in_secs: 25 * 60,
        short_break_time_in_secs: 5 * 60,
        long_break_time_in_secs: 15 * 60,
    };
    let _csv_file = Arc::new(Mutex::new(CsvFile::new("pom-record.csv".to_string())));
    let _record = Record::new(_csv_file);
    _record.initialize();
    let mut _pomodoro: Pomodoro;
    let mut _cli = CLI::new();
    let cli_receiver = _cli.chan_sender();
    let mut no_of_finished_pomodoros = 0;
    match _record.no_of_finished_pomodoros_from_record() {
        Some(no) => {
            _pomodoro = Pomodoro::continue_from(no, config);
            no_of_finished_pomodoros = no;
        }
        None => {
            _pomodoro = Pomodoro::new(config);
        }
    }
    _pomodoro.add_observer(&_record);
    let pom_receiver = _pomodoro.chan_sender();
    _cli.register_receiver(pom_receiver);
    _pomodoro.register_receiver(cli_receiver);
    thread::spawn(move || {
        _cli.start(no_of_finished_pomodoros);
    });
    _pomodoro.listen_loop();
}
