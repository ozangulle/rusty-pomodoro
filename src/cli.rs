use crate::Pomodoro;
use crate::PomodoroStates;
use crate::Observer;

use std::io::{stdin,stdout,Write};

pub struct CLI {}

impl CLI {
    pub fn new() -> CLI {
        CLI {}
    }

    pub fn start(&self, p: &Pomodoro) {
        print!("\x07");
        println!("You have finished {} pomodoros today", p.finished_pomodoros());
        let next_state = &p.next_state;
        if *next_state == PomodoroStates::Pomodoro {
            print!("Starting a new pomodoro. ");
        } else if *next_state == PomodoroStates::ShortBreak {
            print!("Let's have a short break. ")
        } else if *next_state == PomodoroStates::LongBreak {
            print!("Let's have a long break. ")
        }
        self.pause();
    }

    fn pause(&self) {
        let mut s=String::new();
        print!("Please press enter...");
        let _=stdout().flush();
        stdin().read_line(&mut s);
    }
}

impl Observer for CLI {
    fn callback(&self, p: &Pomodoro) {
        self.start(p);
    }
}
