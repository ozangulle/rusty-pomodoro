use crate::pomodoro::PomodoroStates;

pub trait Observer {
    fn callback(&self, next_state: PomodoroStates, finished_pomodoros: u32);
}
