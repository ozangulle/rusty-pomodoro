use crate::pomodoro::Pomodoro;

pub trait Observer {
    fn callback(&self, p: &Pomodoro);
}
