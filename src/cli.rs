use crate::communication::*;
use crate::pomodoro::PomodoroStates;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

pub struct CLI {
    ui_sender: Option<Sender<UIChannel>>,
    pom_receiver: Option<Receiver<PomodoroChannel>>,
}

impl CLI {
    pub fn new() -> CLI {
        CLI {
            ui_sender: None,
            pom_receiver: None,
        }
    }

    pub fn start(&self, finished_pomodoros: u32) {
        self.ask_for_ack(PomodoroStates::Pomodoro, finished_pomodoros);
    }

    fn ask_for_ack(&self, next_state: PomodoroStates, finished_pomodoros: u32) {
        print!("{}[2J", 27 as char);
        print!("\x07");
        println!("You have finished {} pomodoros today", finished_pomodoros);
        if next_state == PomodoroStates::Pomodoro {
            print!("Starting a new pomodoro. ");
        } else if next_state == PomodoroStates::ShortBreak {
            print!("Let's have a short break. ")
        } else if next_state == PomodoroStates::LongBreak {
            print!("Let's have a long break. ")
        }
        self.pause();
        match self.ui_sender.as_ref() {
            Some(channel) => {
                match channel.send(UIChannel::Proceed) {
                    Ok(_) => (),
                    Err(_) => (),
                }
            },
            None => (),
        }
        self.listening_loop();
    }

    fn pause(&self) {
        let mut s = String::new();
        print!("Please press enter...");
        let _ = stdout().flush();
        stdin().read_line(&mut s);
    }

    fn play_animation(&self, remaining_secs: u64) {
        let mut frame: usize = 0;
        let animation = vec!["|", "/", "-", "\\", "."];
        while frame < 5 {
            print!("\r");
            if remaining_secs > 60 {
                print!(
                    "{} {} minutes remaining ",
                    animation[frame], self.remaining_minutes(remaining_secs)
                );
            } else {
                print!(
                    "{} {} seconds remaining ",
                    animation[frame], remaining_secs
                );
            }
            stdout().flush();
            thread::sleep(Duration::from_secs(1));
            frame = frame + 1;
        }
        self.listening_loop();
    }

    fn remaining_minutes(&self, remaining_secs: u64) -> u64 {
        (remaining_secs as f64 / 60 as f64).ceil() as u64
    }

    fn listening_loop(&self) {
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
}

impl ConcSender<UIChannel> for CLI {
    fn chan_sender(&mut self) -> Receiver<UIChannel> {
        let (sender, receiver) = channel();
        self.ui_sender = Some(sender);
        receiver
    }
}

impl ConcReceiver<PomodoroChannel> for CLI {
    fn register_receiver(&mut self, receiver: Receiver<PomodoroChannel>) {
        self.pom_receiver = Some(receiver);
    }
}
