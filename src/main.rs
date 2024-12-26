use std::io;

use ratatui::{DefaultTerminal, Frame};

#[derive(Default)]
pub struct App {
    counter: u8,
    exit: bool,
}
impl App {
    fn run(&self, terminal: &mut DefaultTerminal) -> Result<(), io::Error> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame<'_>) {
        todo!()
    }

    fn handle_events(&self) -> io::Result<()> {
        todo!()
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
