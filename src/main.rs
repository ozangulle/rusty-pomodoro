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
    let mut _pomodoro = Pomodoro::new();
    let _cli = CLI::new();
    let _record = Record::new();
    _pomodoro.add_observer(&_record);
    _pomodoro.add_observer(&_cli);
    _pomodoro.proceed();
}
