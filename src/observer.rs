use crate::Pomodoro;

pub trait Observer {
    fn callback(&self, p: &Pomodoro);
}
