use crate::observer::Observer;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone)]
pub enum PomodoroStates {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

pub struct PomodoroConfig {
    pub pomodoro_time_in_secs: u64,
    pub short_break_time_in_secs: u64,
    pub long_break_time_in_secs: u64,
    pub max_pomodoros: u32,
}

pub struct Pomodoro<'a> {
    pub finished_pomodoros: u32,
    no_of_breaks: u8,
    pomodoro_time_in_secs: u64,
    short_break_time_in_secs: u64,
    long_break_time_in_secs: u64,
    pub next_state: PomodoroStates,
    observers: Vec<&'a Observer>,
    max_pomodoros: u32,
}

impl<'a> Pomodoro<'a> {
    pub fn new(config: PomodoroConfig) -> Pomodoro<'a> {
        Pomodoro {
            finished_pomodoros: 0,
            no_of_breaks: 0,
            pomodoro_time_in_secs: config.pomodoro_time_in_secs,
            short_break_time_in_secs: config.short_break_time_in_secs,
            long_break_time_in_secs: config.long_break_time_in_secs,
            next_state: PomodoroStates::Pomodoro,
            observers: Vec::new(),
            max_pomodoros: config.max_pomodoros,
        }
    }

    pub fn continue_from(no_of_pomodoros: u32, config: PomodoroConfig) -> Pomodoro<'a> {
        Pomodoro {
            finished_pomodoros: no_of_pomodoros,
            ..Pomodoro::new(config)
        }
    }

    pub fn proceed(&mut self) {
        while self.max_pomodoros == 0 || self.finished_pomodoros < self.max_pomodoros {
            self.notify();
            match self.next_state {
                PomodoroStates::Pomodoro => {
                    if self.no_of_breaks == 3 {
                        self.next_state = PomodoroStates::LongBreak;
                    } else {
                        self.next_state = PomodoroStates::ShortBreak;
                    }
                    self.wait_for_seconds(self.pomodoro_time_in_secs);
                    self.finished_pomodoros += 1;
                },
                PomodoroStates::ShortBreak => {
                    self.next_state = PomodoroStates::Pomodoro;
                    self.no_of_breaks = self.no_of_breaks + 1;
                    self.wait_for_seconds(self.short_break_time_in_secs);
                },
                PomodoroStates::LongBreak => {
                    self.next_state = PomodoroStates::Pomodoro;
                    self.no_of_breaks = 0;
                    self.wait_for_seconds(self.long_break_time_in_secs);
                }
            }
        }
    }

    pub fn finished_pomodoros(&self) -> String {
        self.finished_pomodoros.to_string()
    }

    pub fn add_observer(&mut self, observer: &'a impl Observer) {
        self.observers.push(observer);
    }
    
    fn wait_for_seconds(&self, seconds: u64) {
        thread::sleep(Duration::from_secs(seconds));
    }
 
    fn notify(&self) {
        for observer in self.observers.iter() {
            observer.callback(&self);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pomodoro::*;

    fn create_pomodoro_with_max_pomodoros(max_pomodoros: u32) -> Pomodoro<'static> {
        let config = PomodoroConfig{
            pomodoro_time_in_secs: 0,
            short_break_time_in_secs: 0,
            long_break_time_in_secs: 0,
            max_pomodoros: max_pomodoros,
        };
        Pomodoro::new(config)
    }

    #[test]
    fn state_jumps_from_pomodoro_to_short_break() {
        let mut pom = create_pomodoro_with_max_pomodoros(1);
        pom.proceed();
        assert!(pom.next_state == PomodoroStates::ShortBreak);
        assert_eq!(pom.finished_pomodoros(), "1");
    }

    #[test]
    fn after_four_pomodoros_come_a_long_break() {
        let mut pom = create_pomodoro_with_max_pomodoros(4);
        pom.proceed();
        assert!(pom.next_state == PomodoroStates::LongBreak);
        assert!(pom.finished_pomodoros == 4);
    }

    #[test]
    fn continue_from_existing_record() {
        let config = PomodoroConfig{
            pomodoro_time_in_secs: 0,
            short_break_time_in_secs: 0,
            long_break_time_in_secs: 0,
            max_pomodoros: 4,
        };
        let pom = Pomodoro::continue_from(12, config);
        assert!(pom.finished_pomodoros == 12);
    }
}