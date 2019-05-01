use crate::communication::*;
use crate::observers::*;
use crate::pomodoro::PomodoroStates;
use crate::pomodoro::PomodoroConfig;
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
            pomodoro_time_in_secs: config.pomodoro_time_in_secs,
            short_break_time_in_secs: config.short_break_time_in_secs,
            long_break_time_in_secs: config.long_break_time_in_secs,
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
        match self.ui_receiver.as_ref() {
            Some(channel) => match channel.recv() {
                Ok(message) => match message {
                    UIChannel::Proceed => self.run_pom_cycle(),
                    UIChannel::Cancel => (),
                },
                Err(_) => (),
            },
            None => (),
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
                self.no_of_breaks = self.no_of_breaks + 1;
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
        match self.pom_sender.as_ref() {
            Some(channel) => {
                channel.send(PomodoroChannel::Update(remaining_secs));
            }
            None => (),
        }
    }

    fn notify(&self) {
        for observer in self.state_observers.iter() {
            observer.callback(self.next_state.clone(), self.finished_pomodoros);
        }
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
    use crate::observers::Observer;
    use crate::communication::UIChannel;
    use crate::pomodoro::*;
    #[macro_use]
    use simulacrum::*;
    use std::sync::mpsc::channel;
    use std::sync::mpsc::{Receiver, Sender};
    use std::thread;
    use std::time::Duration;

    create_mock! {
        impl IOComponent for MockIO (self) {
            expect_ask_to_continue("ask_to_continue"):
            fn ask_to_continue(&self, next_state: PomodoroStates, finished_pomodoros: u32)-> (bool, Sender<bool>);
        }
    }

    fn pom_config() -> PomodoroConfig {
        PomodoroConfig {
            pomodoro_time_in_secs: 0,
            short_break_time_in_secs: 0,
            long_break_time_in_secs: 0,
        }
    }

    #[test]
    fn state_jumps_from_pomodoro_to_short_break() {
        let pom_config = pom_config();
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
        let pom_config = pom_config();
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
        let pom_config = pom_config();
        let mock_io = MockIO::new();
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
        let pom_config = pom_config();
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
        let pom_config = pom_config();
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
}
