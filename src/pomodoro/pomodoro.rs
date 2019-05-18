use crate::communication::*;
use crate::observers::*;
use crate::pomodoro::PomodoroConfig;
use crate::pomodoro::PomodoroStates;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct Pomodoro<'a> {
    pub finished_pomodoros: u32,
    no_of_breaks: u8,
    pomodoro_time_in_secs: u64,
    short_break_time_in_secs: u64,
    long_break_time_in_secs: u64,
    pub next_state: PomodoroStates,
    pub current_state: PomodoroStates,
    pub state_observers: Vec<&'a dyn Observer>,
    ui_receiver: Option<Receiver<UIChannel>>,
    pom_sender: Option<Sender<PomodoroChannel>>,
}

impl<'a> Pomodoro<'a> {
    pub fn new(config: PomodoroConfig) -> Pomodoro<'a> {
        Pomodoro {
            finished_pomodoros: 0,
            no_of_breaks: 0,
            pomodoro_time_in_secs: Pomodoro::convert_minutes_to_seconds(
                config.pomodoro_time_in_mins,
            ),
            short_break_time_in_secs: Pomodoro::convert_minutes_to_seconds(
                config.short_break_time_in_mins,
            ),
            long_break_time_in_secs: Pomodoro::convert_minutes_to_seconds(
                config.long_break_time_in_mins,
            ),
            current_state: PomodoroStates::Pomodoro,
            next_state: PomodoroStates::Pomodoro,
            state_observers: Vec::new(),
            ui_receiver: None,
            pom_sender: None,
        }
    }

    pub fn continue_from(no_of_pomodoros: u32, config: PomodoroConfig) -> Pomodoro<'a> {
        Pomodoro {
            finished_pomodoros: no_of_pomodoros,
            ..Pomodoro::new(config)
        }
    }

    pub fn add_observer(&mut self, observer: &'a impl Observer) {
        self.state_observers.push(observer);
    }

    pub fn listen_loop(&mut self) {
        if let Some(channel) = self.ui_receiver.as_ref() {
             if let Ok(message) = channel.recv() {
                 match message {
                    UIChannel::Proceed => self.run_pom_cycle(),
                    UIChannel::Cancel => (),
                }
            }
        }
    }

    fn run_pom_cycle(&mut self) {
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
            }
            PomodoroStates::ShortBreak => {
                self.next_state = PomodoroStates::Pomodoro;
                self.current_state = PomodoroStates::ShortBreak;
                self.no_of_breaks += 1;
                self.wait_for_seconds(self.short_break_time_in_secs);
            }
            PomodoroStates::LongBreak => {
                self.next_state = PomodoroStates::Pomodoro;
                self.current_state = PomodoroStates::LongBreak;
                self.no_of_breaks = 0;
                self.wait_for_seconds(self.long_break_time_in_secs);
            }
        }
        self.notify();
        match self.pom_sender.as_ref() {
            Some(channel) => {
                channel.send(PomodoroChannel::Completed(
                    self.next_state.clone(),
                    self.finished_pomodoros,
                ));
                self.listen_loop();
            }
            None => (),
        }
    }

    fn wait_for_seconds(&self, seconds: u64) {
        let mut remaining_secs = seconds;
        let notify_every = 5;
        self.send_update(remaining_secs);
        while remaining_secs > 5 {
            thread::sleep(Duration::from_secs(notify_every));
            remaining_secs -= notify_every;
            self.send_update(remaining_secs);
        }
    }

    fn send_update(&self, remaining_secs: u64) {
        if let Some(channel) = self.pom_sender.as_ref() {
            channel.send(PomodoroChannel::Update(remaining_secs));
        }
    }

    fn notify(&self) {
        for observer in self.state_observers.iter() {
            observer.callback(self.next_state.clone(), self.finished_pomodoros);
        }
    }

    fn convert_minutes_to_seconds(minutes: f32) -> u64 {
        (minutes * (60 as f32)) as u64
    }
}

impl<'a> ConcSender<PomodoroChannel> for Pomodoro<'a> {
    fn chan_sender(&mut self) -> Receiver<PomodoroChannel> {
        let (sender, receiver) = channel();
        self.pom_sender = Some(sender);
        receiver
    }
}

impl<'a> ConcReceiver<UIChannel> for Pomodoro<'a> {
    fn register_receiver(&mut self, receiver: Receiver<UIChannel>) {
        self.ui_receiver = Some(receiver);
    }
}

#[cfg(test)]
mod tests {
    extern crate simulacrum;
    use crate::communication::*;
    use crate::observers::Observer;
    use crate::pomodoro::*;
    #[macro_use]
    use simulacrum::*;
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    fn zero_time_pom_config() -> PomodoroConfig {
        PomodoroConfig {
            pomodoro_time_in_mins: 0 as f32,
            short_break_time_in_mins: 0 as f32,
            long_break_time_in_mins: 0 as f32,
        }
    }

    #[test]
    fn state_jumps_from_pomodoro_to_short_break() {
        let pom_config = zero_time_pom_config();
        let mut pom = Pomodoro::new(pom_config);
        let (sender, receiver) = channel();
        pom.register_receiver(receiver);
        thread::spawn(move || {
            thread::sleep(Duration::from_micros(10));
            sender.send(UIChannel::Proceed);
        });
        pom.listen_loop();
        assert_eq!(pom.next_state, PomodoroStates::ShortBreak);
        assert_eq!(pom.finished_pomodoros.to_string(), "1");
    }

    #[test]
    fn after_four_pomodoros_come_a_long_break() {
        let pom_config = zero_time_pom_config();
        let mut pom = Pomodoro::new(pom_config);
        let (sender, receiver) = channel();
        pom.register_receiver(receiver);
        pom.chan_sender();
        thread::spawn(move || {
            let no_of_proceedings_till_long_break = 7;
            for _ in 0..no_of_proceedings_till_long_break {
                thread::sleep(Duration::from_micros(10));
                sender.send(UIChannel::Proceed);
            }
        });
        pom.listen_loop();
        assert_eq!(pom.next_state, PomodoroStates::LongBreak);
        assert_eq!(pom.finished_pomodoros, 4);
    }

    #[test]
    fn continue_from_existing_record() {
        let pom_config = zero_time_pom_config();
        let pom = Pomodoro::continue_from(12, pom_config);
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
        let pom_config = zero_time_pom_config();
        let mut pom = Pomodoro::new(pom_config);
        assert_eq!(pom.state_observers.len(), 0);
        pom.add_observer(&observer);
        assert_eq!(pom.state_observers.len(), 1);
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
        observer
            .expect_callback()
            .called_once()
            .with(params!(PomodoroStates::ShortBreak, 1));
        let pom_config = zero_time_pom_config();
        let mut pom = Pomodoro::new(pom_config);
        pom.add_observer(&observer);
        let (sender, receiver) = channel();
        pom.register_receiver(receiver);
        thread::spawn(move || {
            thread::sleep(Duration::from_micros(10));
            sender.send(UIChannel::Proceed);
        });
        pom.listen_loop();
    }

    #[test]
    fn updates_are_sent_correctly() {
        let mut pom = Pomodoro::new(PomodoroConfig {
            pomodoro_time_in_mins: 0.2 as f32,
            short_break_time_in_mins: 0 as f32,
            long_break_time_in_mins: 0 as f32,
        });
        let (sender, receiver) = channel();
        pom.register_receiver(receiver);
        let pom_receiver = pom.chan_sender();
        let mut actual_results: Vec<(u64)> = vec![];
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_micros(10));
            sender.send(UIChannel::Proceed);
            for _ in 0..3 {
                match pom_receiver.recv().unwrap() {
                    PomodoroChannel::Update(remaining_secs) => actual_results.push(remaining_secs),
                    PomodoroChannel::Completed(_next_state, _finished_pomodoros) => (),
                }
            }
            actual_results
        });
        pom.listen_loop();
        let actual_results = handle.join().unwrap();
        assert_eq!(actual_results[0], 12 as u64);
        assert_eq!(actual_results[1], 7 as u64);
        assert_eq!(actual_results[2], 2 as u64);
    }
}
