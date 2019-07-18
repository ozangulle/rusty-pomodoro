use rusty_pomodoro::communication::*;
use rusty_pomodoro::config::YamlConfig;
use rusty_pomodoro::files::*;
use rusty_pomodoro::pomodoro_core::Pomodoro;
use rusty_pomodoro::pomodoro_core::PomodoroConfig;
use rusty_pomodoro::record::Record;
use rusty_pomodoro::ui::*;
use rusty_pomodoro::userinterface::UserInterface;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let config = PomodoroConfig {
        pomodoro_time_in_mins: 25_f32,
        short_break_time_in_mins: 5_f32,
        long_break_time_in_mins: 15_f32,
    };
    let filename_and_location: (String, String) = get_record_name_and_collection("rp-config.yml");
    let record = Record::new(Arc::new(Mutex::new(CsvFile::new(
        filename_and_location.0,
        filename_and_location.1,
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
    let mut ui = UserInterface::new(Arc::new(SimpleTUI::new()));
    let cli_receiver = ui.chan_sender();
    let pom_receiver = pomodoro.chan_sender();
    ui.register_receiver(pom_receiver);
    pomodoro.register_receiver(cli_receiver);
    thread::spawn(move || {
        ui.start(no_of_finished_pomodoros);
    });
    pomodoro.listen_loop();
}

fn get_record_name_and_collection(config_filename: &str) -> (String, String) {
    let default_filename = "pom-record";
    let default_location = "./";
    let mut config = YamlConfig::new(config_filename);
    match config.parse() {
        Ok(()) => {
            let filename: &str;
            let location: &str;
            match config.record_filename() {
                Some(name) => filename = name,
                None => filename = default_filename,
            }
            match config.record_location() {
                Some(loc) => location = loc,
                None => location = default_location,
            }
            (location.to_string(), filename.to_string())
        }
        Err(_) => (default_location.to_string(), default_filename.to_string()),
    }
}
