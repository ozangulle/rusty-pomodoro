use rusty_pomodoro::ui::UI;
use rusty_pomodoro::communication::*;
use rusty_pomodoro::files::*;
use rusty_pomodoro::pomodoro::Pomodoro;
use rusty_pomodoro::pomodoro::PomodoroConfig;
use rusty_pomodoro::record::Record;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let config = PomodoroConfig {
        pomodoro_time_in_mins: 25 as f32,
        short_break_time_in_mins: 5 as f32,
        long_break_time_in_mins: 15 as f32,
    };
    let record = Record::new(Arc::new(Mutex::new(CsvFile::new(
        "pom-record.csv".to_string(),
    ))));
    record.initialize();
    let mut pomodoro: Pomodoro;
    let mut no_of_finished_pomodoros = 0;
    match record.no_of_finished_pomodoros_from_record() {
        Some(no) => {
            pomodoro = Pomodoro::continue_from(no, config);
            no_of_finished_pomodoros = no;
        }
        None => {
            pomodoro = Pomodoro::new(config);
        }
    }
    pomodoro.add_observer(&record);
    let mut cli = UI::new();
    let cli_receiver = cli.chan_sender();
    let pom_receiver = pomodoro.chan_sender();
    cli.register_receiver(pom_receiver);
    pomodoro.register_receiver(cli_receiver);
    thread::spawn(move || {
        cli.start(no_of_finished_pomodoros);
    });
    pomodoro.listen_loop();
}
