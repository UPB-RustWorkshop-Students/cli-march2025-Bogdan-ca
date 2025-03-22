use crate::app::{App, AppResult, InputMode};
use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match app.input_mode {
        InputMode::Normal => match key_event.code {
            // Exit application
            KeyCode::Char('q') => {
                app.running = false;
            }
            KeyCode::Esc => {
                app.running = false;
            }
            
            // Navigation - up/down to select cities
            KeyCode::Up => {
                app.previous_city();
            }
            KeyCode::Down => {
                app.next_city();
            }
            
            // Add a new city - enter edit mode
            KeyCode::Char('a') => {
                app.enter_edit_mode();
            }
            
            // Delete the selected city
            KeyCode::Char('d') => {
                if !app.cities.is_empty() {
                    app.cities.remove(app.selected_city);
                    if app.selected_city >= app.cities.len() && !app.cities.is_empty() {
                        app.selected_city = app.cities.len() - 1;
                    }
                }
            }
            
            // Refresh weather data for current city
            KeyCode::Char('r') => {
                // If you have an async refresh function, you'll need to handle this differently
                // This is just a placeholder
                // app.refresh_weather();
            }
            
            _ => {}
        },
        
        InputMode::Editing => match key_event.code {
            // Cancel edit
            KeyCode::Esc => {
                app.exit_edit_mode();
            }
            
            // Submit new city
            KeyCode::Enter => {
                app.add_city();
            }
            
            // Backspace - delete character
            KeyCode::Backspace => {
                app.delete_char();
            }
            
            // Type characters
            KeyCode::Char(c) => {
                app.handle_input(c);
            }
            
            _ => {}
        }
    }
    
    Ok(())
}