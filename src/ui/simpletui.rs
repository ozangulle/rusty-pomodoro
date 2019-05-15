use crate::ui::Output;
use crate::uimessages::UIMessages;
use crossterm::{terminal, ClearType, Color, Colored, Terminal};
use std::io::{stdin, stdout, Write};
use std::thread;
use std::time::Duration;

pub struct SimpleTUI {
    terminal: Terminal,
}

impl SimpleTUI {
    pub fn new() -> SimpleTUI {
        SimpleTUI {
            terminal: terminal(),
        }
    }

    fn play_animation(&self, ui_message: String) {
        let mut frame: usize = 0;
        let animation = vec!["| ", "/ ", "- ", "\\ ", ". "];
        while frame < 5 {
            self.terminal.clear(ClearType::CurrentLine);
            print!("\r");
            self.print_styled_message(animation[frame], Colored::Fg(Color::Yellow));
            self.print_styled_message(ui_message.as_str(), Colored::Fg(Color::Cyan));
            stdout().flush();
            if frame < 4 {
                thread::sleep(Duration::from_secs(1));
            }
            frame = frame + 1;
        }
    }

    fn print_state_message(&self, state_message: String) {
        self.print_styled_message(state_message.as_str(), Colored::Fg(Color::White));
    }

    fn print_summary_message(&self, summary_message: String) {
        self.terminal.clear(ClearType::All);
        self.new_line_styled_message(summary_message.as_str(), Colored::Fg(Color::White));
    }

    fn ask_for_input(&self) {
        let mut s = String::new();
        self.print_styled_message(" Please press enter...", Colored::Fg(Color::White));
        let _ = stdout().flush();
        stdin().read_line(&mut s);
    }

    fn print_styled_message(&self, message: &str, style: Colored) {
        print!("{}{}", style, message);
    }

    fn new_line_styled_message(&self, message: &str, style: Colored) {
        println!("{}{}", style, message);
    }
}

impl Output for SimpleTUI {
    fn display(&self, ui_message: UIMessages) {
        match ui_message {
            UIMessages::StateMessage(message) => self.print_state_message(message),
            UIMessages::InputMessage() => self.ask_for_input(),
            UIMessages::SummaryMessage(message) => self.print_summary_message(message),
            UIMessages::ProgressMessage(message) => self.play_animation(message),
        }
    }
}
