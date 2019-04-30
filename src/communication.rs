use std::sync::mpsc::{Receiver};
use crate::pomodoro::PomodoroStates;

pub enum PomodoroChannel {
    Update(u64),
    Completed(PomodoroStates, u32)
}

pub enum UIChannel {
    Proceed,
    Cancel,
}

pub trait ConcSender<T> {
    fn chan_sender(&mut self)  -> Receiver<T>;
}

pub trait ConcReceiver<T> {
    fn register_receiver(&mut self, receiver: Receiver<T>) -> ();
}
