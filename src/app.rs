use std::io;

use ratatui::{
    DefaultTerminal, Frame
};

use crate::connections::{Connections};

/// Struct for the main data for the App.
pub struct App {
    solution: Connections,
    has_won: bool,
    author: String
}

impl Default for App {
    fn default() -> Self {
        Self {
            solution: Connections::new(),
            has_won: false,
            author: String::from("")
        }
    }
}

impl App {

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.has_won {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!()
    }
}
