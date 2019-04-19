use crate::pomodoro::Pomodoro;
use crate::pomodoro::PomodoroStates;
use crate::observer::Observer;

use std::thread;
use std::time::Duration;
use std::io::{stdin,stdout,Write};
use std::sync::mpsc;

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
        self.play_animation();
    }

    fn pause(&self) {
        let mut s=String::new();
        print!("Please press enter...");
        let _=stdout().flush();
        stdin().read_line(&mut s);
    }

    fn play_animation(&self) {
        thread::spawn(move || {
            let mut frame: usize = 0;
            let animation = vec!["|", "/", "-", "\\"];
            while frame < 5 {
                if frame == 4 {
                    frame = 0;
                }
                thread::sleep(Duration::from_secs(1));
                print!("\r");
                print!("{}", animation[frame]);
                stdout().flush();
                frame = frame + 1;
            }
        });
    }
}

impl Observer for CLI {
    fn callback(&self, p: &Pomodoro) {
        self.start(p);
    }
}
