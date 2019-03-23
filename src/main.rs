mod pomodoro;
mod cli;
mod observer;
mod record;
use pomodoro::Pomodoro;
use pomodoro::PomodoroStates;
use cli::CLI;
use observer::Observer;
use record::Record;

fn main() {
    let _record = Record::new();
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
