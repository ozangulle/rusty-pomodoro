use crate::observer::Observer;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone)]
pub enum PomodoroStates {
    Pomodoro,
    ShortBreak,
    LongBreak
}

pub struct Pomodoro<'a> {
    finished_pomodoros: u32,
    no_of_breaks: u8,
    pomodoro_time_in_secs: u64,
    short_break_time_in_secs: u64,
    long_break_time_in_secs: u64,
    next_state: PomodoroStates,
    observers: Vec<&'a Observer>
}

impl<'a> Pomodoro<'a> {
    pub fn new() -> Pomodoro<'a> {
        Pomodoro {
            finished_pomodoros: 0,
            no_of_breaks: 0,
            pomodoro_time_in_secs: 25 * 60,
            short_break_time_in_secs: 5 * 60,
            long_break_time_in_secs: 15 * 60,
            next_state: PomodoroStates::Pomodoro,
            observers: Vec::new()
        }
    }

    pub fn proceed(&mut self) {
        loop {
            self.notify();
            match self.next_state {
                PomodoroStates::Pomodoro => {
                    if self.no_of_breaks == 3 {
                        self.next_state = PomodoroStates::LongBreak;
                    } else {
                        self.next_state = PomodoroStates::ShortBreak;
                    }
                    self.start_pomodoro();
                    self.finished_pomodoros += 1;
                },
                PomodoroStates::ShortBreak => {
                    self.next_state = PomodoroStates::Pomodoro;
                    self.no_of_breaks = self.no_of_breaks + 1;
                    self.start_short_break();
                },
                PomodoroStates::LongBreak => {
                    self.next_state = PomodoroStates::Pomodoro;
                    self.no_of_breaks = 0;
                    self.start_long_break();
                }
            }
        }
    }

    pub fn next_state(&self) -> PomodoroStates {
        return self.next_state.clone();
    }

    pub fn finished_pomodoros(&self) -> String {
        return self.finished_pomodoros.to_string();
    }

    pub fn add_observer(&mut self, observer: &'a impl Observer) {
        self.observers.push(observer);
    }
    
    fn start_pomodoro(&self) {
        thread::sleep(Duration::from_secs(self.pomodoro_time_in_secs));
    }

    fn start_short_break(&self) {
        thread::sleep(Duration::from_secs(self.short_break_time_in_secs));
    }

    fn start_long_break(&self) {
        thread::sleep(Duration::from_secs(self.long_break_time_in_secs));
    }
 
    fn notify(&self) {
        let p = self.clone();
        for observer in self.observers.iter() {
            observer.callback(&p);
        }
    }

    fn clone(&self) -> Pomodoro {
        Pomodoro {
            finished_pomodoros: self.finished_pomodoros,
            no_of_breaks: self.no_of_breaks,
            pomodoro_time_in_secs: self.pomodoro_time_in_secs,
            short_break_time_in_secs: self.short_break_time_in_secs,
            long_break_time_in_secs: self.long_break_time_in_secs,
            next_state: self.next_state.clone(),
            observers: Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pomodoro::Pomodoro;
    use crate::pomodoro::PomodoroStates;
    #[test]
    fn state_jumps_from_pomodoro_to_short_break() {
        let mut pom = Pomodoro {
            finished_pomodoros: 0,
            no_of_breaks: 0,
            pomodoro_time_in_secs: 0,
            short_break_time_in_secs: 0,
            long_break_time_in_secs: 0,
            next_state: PomodoroStates::Pomodoro,
        };
        pom.proceed();
        assert!(pom.next_state() == PomodoroStates::ShortBreak);
        assert_eq!(pom.finished_pomodoros(), "1");
    }

    #[test]
    fn after_four_pomodoros_come_a_long_break() {
        let mut pom = Pomodoro {
            finished_pomodoros: 0,
            no_of_breaks: 0,
            pomodoro_time_in_secs: 0,
            short_break_time_in_secs: 0,
            long_break_time_in_secs: 0,
            next_state: PomodoroStates::Pomodoro,
        };
        // Pomodoro
        pom.proceed();
        // Short break
        pom.proceed();
        // Pomodoro
        pom.proceed();
        // Short break
        pom.proceed();
        // Pomodoro
        pom.proceed();
        // Short break
        pom.proceed();
        // Pomodoro
        pom.proceed();
        assert!(pom.next_state() == PomodoroStates::LongBreak);
        assert_eq!(pom.finished_pomodoros(), "4");
    }
}