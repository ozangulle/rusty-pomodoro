use crate::pomodoro::Pomodoro;
use crate::pomodoro::PomodoroStates;

pub trait Observer {
    fn callback(&self, next_state: PomodoroStates, finished_pomodoros: u32);
}

pub trait UpdateState {
    fn update_state(&self, state: PomodoroStates, remaining_secs: u64);
}
