use std::io::{self, stdout};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, style::Color};
use std::time::{Duration, Instant};

#[derive(PartialEq)]
pub enum Chrono {
    Started,
    Paused,
    Stopped,
}

pub struct App {
    pub input: String,
    pub chrono_state: Chrono,
	pub messages: Vec<String>,
    pub list_state: ListState,
    pub start_time: Option<Instant>,
    pub focus_duration: Duration,
}

impl App {
    pub fn new() -> App {
        App {
            input: String::new(),
            chrono_state: Chrono::Stopped, // On démarre à l'arrêt
            messages: Vec::new(),
            list_state: ListState::default(),
            start_time: None,
            focus_duration: Duration::from_secs(25 * 60),   
        }
    }

	pub fn start_chrono(&mut self) {
		self.chrono_state = Chrono::Started;
		self.start_time = Some(Instant::now());
	}

	pub fn stop_chrono(&mut self) {
		self.chrono_state = Chrono::Stopped;
		self.start_time = None;
	}

    pub fn change_duration(&mut self, mins: u64) {
        self.focus_duration = Duration::from_secs(mins * 60);
    }

    pub fn scroll_to_bottom(&mut self) {
        if !self.messages.is_empty() {
            self.list_state.select(Some(self.messages.len() - 1));
        }
    }

    pub fn scroll_up(&mut self) {
        if !self.messages.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => if i == 0 { 0 } else { i - 1 },
                None => 0,
            };
            self.list_state.select(Some(i)); // on va changer la variable sélectionné afin de pointer vers un autre message
        }
    }

    pub fn scroll_down(&mut self) {
        if !self.messages.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) => {
                    if i >= self.messages.len() - 1 {
                        self.messages.len() - 1
                    } else {
                        i + 1
                    }
                }
                None => 0,
            };
            self.list_state.select(Some(i));
        }
    }
}