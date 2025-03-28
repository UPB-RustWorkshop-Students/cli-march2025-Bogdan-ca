use crate::app::{App, AppResult};
use crate::event::EventsPublisher;
use crate::ui;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
#[derive(Debug)]
pub struct Tui<B: Backend> {
    /// Interface to the Terminal.
    terminal: Terminal<B>,
    /// Terminal event handler.
    pub events: EventsPublisher,
}

impl<B: Backend> Tui<B> {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: Terminal<B>, events: EventsPublisher) -> Self {
        Self { terminal, events}
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// Returns Ok() is no errors occured, Err() otherwhise
    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        // Set cursor visibility based on input mode
        if app.input_mode == crate::app::InputMode::Editing {
            self.terminal.show_cursor()?;
        } else {
            self.terminal.hide_cursor()?;
        }
        
        // Draw the interface - remove the generic parameter B
        self.terminal.draw(|frame| {
            ui::render(app, frame);  // No generic parameter here!
        })?;
        
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
