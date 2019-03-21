use crate::Observer;
use crate::Pomodoro;

pub struct Record {}

impl Record {
    pub fn new() -> Record {
        Record {}
    }

    fn process(&self, _p: &Pomodoro) {
        // No implementation yet
    }
}

impl Observer for Record {
    fn callback(&self, p: &Pomodoro) {
        self.process(p);
    }
}
