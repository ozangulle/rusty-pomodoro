use crate::communication::*;
use crate::pomodoro::PomodoroStates;
use crate::ui::Output;
use crate::uimessages::UIMessages;
use crossterm::{terminal, ClearType, Color, Colored, Terminal};
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct UserInterface {
    ui_sender: Option<Sender<UIChannel>>,
    pom_receiver: Option<Receiver<PomodoroChannel>>,
    output: Arc<dyn Output>,
}

impl UserInterface {
    pub fn new(output: Arc<dyn Output>) -> UserInterface {
        UserInterface {
            ui_sender: None,
            pom_receiver: None,
            output,
        }
    }

    pub fn start(&mut self, finished_pomodoros: u32) {
        self.ask_for_ack(PomodoroStates::Pomodoro, finished_pomodoros);
    }

    fn ask_for_ack(&mut self, next_state: PomodoroStates, finished_pomodoros: u32) {
        self.print_finished_pomodoro_str(finished_pomodoros);
        if next_state == PomodoroStates::Pomodoro {
            self.output.display(UIMessages::StateMessage(
                "Starting a new pomodoro.".to_string(),
            ));
        } else if next_state == PomodoroStates::ShortBreak {
            self.output.display(UIMessages::StateMessage(
                "Let's have a short break.".to_string(),
            ));
        } else if next_state == PomodoroStates::LongBreak {
            self.output.display(UIMessages::StateMessage(
                "Let's have a long break.".to_string(),
            ));
        }
        self.wait_for_user_input();
        match self.ui_sender.as_ref() {
            Some(channel) => match channel.send(UIChannel::Proceed) {
                Ok(_) => (),
                Err(_) => (),
            },
            None => (),
        }
        self.listening_loop();
    }

    fn wait_for_user_input(&self) {
        self.output.display(UIMessages::InputMessage());
    }

    fn play_animation(&mut self, remaining_secs: u64) {
        if remaining_secs > 60 {
            self.output.display(UIMessages::ProgressMessage(format!(
                "{} minutes remaining",
                self.remaining_minutes(remaining_secs)
            )));
        } else {
            self.output.display(UIMessages::ProgressMessage(format!(
                "{} seconds remaining",
                remaining_secs
            )));
        }
        self.listening_loop();
    }

    fn remaining_minutes(&self, remaining_secs: u64) -> u64 {
        (remaining_secs as f64 / 60 as f64).ceil() as u64
    }

    fn listening_loop(&mut self) {
        match self.pom_receiver.as_ref() {
            Some(channel) => match channel.recv() {
                Ok(message) => match message {
                    PomodoroChannel::Update(remaining_secs) => self.play_animation(remaining_secs),
                    PomodoroChannel::Completed(next_state, finished_pomodoros) => {
                        self.ask_for_ack(next_state, finished_pomodoros)
                    }
                },
                Err(_) => (),
            },
            None => (),
        }
    }

    fn print_finished_pomodoro_str(&self, finished: u32) {
        let mut finished_string = String::new();
        finished_string.push_str("You have finished ");
        finished_string.push_str(&finished.to_string());
        finished_string.push_str(" pomodoros today.");
        self.output
            .display(UIMessages::SummaryMessage(finished_string));
    }
}

impl ConcSender<UIChannel> for UserInterface {
    fn chan_sender(&mut self) -> Receiver<UIChannel> {
        let (sender, receiver) = channel();
        self.ui_sender = Some(sender);
        receiver
    }
}

impl ConcReceiver<PomodoroChannel> for UserInterface {
    fn register_receiver(&mut self, receiver: Receiver<PomodoroChannel>) {
        self.pom_receiver = Some(receiver);
    }
}
