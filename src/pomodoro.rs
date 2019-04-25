use crate::observers::*;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone, Debug)]
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
    pub current_state: PomodoroStates,
    pub state_observers: Vec<&'a Observer>,
    pub update_observers: Vec<&'a UpdateState>,
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
            current_state: PomodoroStates::Pomodoro,
            next_state: PomodoroStates::Pomodoro,
            state_observers: Vec::new(),
            update_observers: Vec::new(),
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
        match self.next_state {
            PomodoroStates::Pomodoro => {
                if self.no_of_breaks == 3 {
                    self.next_state = PomodoroStates::LongBreak;
                } else {
                    self.next_state = PomodoroStates::ShortBreak;
                }
                self.current_state = PomodoroStates::Pomodoro;
                self.wait_for_seconds(self.pomodoro_time_in_secs);
                self.finished_pomodoros += 1;
            },
            PomodoroStates::ShortBreak => {
                self.next_state = PomodoroStates::Pomodoro;
                self.current_state = PomodoroStates::ShortBreak;
                self.no_of_breaks = self.no_of_breaks + 1;
                self.wait_for_seconds(self.short_break_time_in_secs);
            },
            PomodoroStates::LongBreak => {
                self.next_state = PomodoroStates::Pomodoro;
                self.current_state = PomodoroStates::LongBreak;
                self.no_of_breaks = 0;
                self.wait_for_seconds(self.long_break_time_in_secs);
            }
        }
        self.notify();
    }

    pub fn add_observer(&mut self, observer: &'a impl Observer) {
        self.state_observers.push(observer);
    }

    pub fn add_update_observer(&mut self, observer: &'a impl UpdateState) {
        self.update_observers.push(observer);
    }
    
    fn wait_for_seconds(&self, seconds: u64) {
        let mut remaining_secs = seconds;
        let notify_every = 5;
        self.send_update(remaining_secs);
        while remaining_secs > 0 {
            thread::sleep(Duration::from_secs(notify_every));
            remaining_secs -= notify_every;
            self.send_update(remaining_secs);
        }
    }
 
    fn notify(&self) {
        for observer in self.state_observers.iter() {
            observer.callback(self.next_state.clone(), self.finished_pomodoros);
        }
    }

    fn send_update(&self, seconds_remaining: u64) {
        for observer in self.update_observers.iter() {
            observer.update_state(self.current_state.clone(), seconds_remaining);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate simulacrum;
    use crate::pomodoro::*;
    use crate::observers::{Observer, UpdateState};
    #[macro_use]
    use simulacrum::*;

    fn create_pomodoro_with_max_pomodoros(max_pomodoros: u32) -> Pomodoro<'static> {
        let config = PomodoroConfig{
            pomodoro_time_in_secs: 0,
            short_break_time_in_secs: 0,
            long_break_time_in_secs: 0,
            max_pomodoros,
        };
        Pomodoro::new(config)
    }

    #[test]
    fn state_jumps_from_pomodoro_to_short_break() {
        let mut pom = create_pomodoro_with_max_pomodoros(1);
        pom.proceed();
        assert_eq!(pom.next_state, PomodoroStates::ShortBreak);
        assert_eq!(pom.finished_pomodoros.to_string(), "1");
    }

    #[test]
    fn after_four_pomodoros_come_a_long_break() {
        let mut pom = create_pomodoro_with_max_pomodoros(4);
        let no_of_proceedings_till_long_break = 7;
        for _ in 0..no_of_proceedings_till_long_break {
            pom.proceed();
        }
        assert_eq!(pom.next_state, PomodoroStates::LongBreak);
        assert_eq!(pom.finished_pomodoros, 4);
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
        assert_eq!(pom.finished_pomodoros, 12);
    }

    #[test]
    fn state_observers_are_added() {
        create_mock! {
            impl Observer for MockObserver (self) {
                expect_callback("callback"):
                fn callback(&self, next_state: PomodoroStates, finished_pomodoros: u32);
            }
        }
        let observer = MockObserver::new();
        let mut pom = create_pomodoro_with_max_pomodoros(4);
        assert_eq!(pom.state_observers.len(), 0);
        pom.add_observer(&observer);
        assert_eq!(pom.state_observers.len(), 1);
    }

    #[test]
    fn update_observers_are_added() {
        create_mock! {
            impl UpdateState for MockObserver (self) {
                expect_update_state("update_state"):
                fn update_state(&self, state: PomodoroStates, remaining_secs: u64);
            }
        }
        let observer = MockObserver::new();
        let mut pom = create_pomodoro_with_max_pomodoros(4);
        assert_eq!(pom.update_observers.len(), 0);
        pom.add_update_observer(&observer);
        assert_eq!(pom.update_observers.len(), 1);
    }

    #[test]
    fn state_observers_are_called_correctly() {
        create_mock! {
            impl Observer for MockObserver (self) {
                expect_callback("callback"):
                fn callback(&self, next_state: PomodoroStates, finished_pomodoros: u32);
            }
        }
        let mut observer = MockObserver::new();
        observer.expect_callback()
            .called_once()
            .with(params!(PomodoroStates::ShortBreak, 1));
        let mut pom = create_pomodoro_with_max_pomodoros(4);
        pom.add_observer(&observer);
        pom.proceed();
    }

    #[test]
    fn update_observers_are_called_correctly() {
        create_mock! {
            impl UpdateState for MockObserver (self) {
                expect_update_state("update_state"):
                fn update_state(&self, state: PomodoroStates, remaining_secs: u64);
            }
        }
        let mut observer = MockObserver::new();
        observer.expect_update_state()
            .called_once()
            .with(params!(PomodoroStates::Pomodoro, 0));
        let mut pom = create_pomodoro_with_max_pomodoros(4);
        pom.add_update_observer(&observer);
        pom.proceed();
    }
}