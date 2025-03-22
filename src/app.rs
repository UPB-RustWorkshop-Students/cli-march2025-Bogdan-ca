use std::error;
use ratatui::widgets::ListState;
use crate::connection::CityInfo;

/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

/// Input mode for the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// API key for OpenWeather
    pub api_key: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current input value when adding a new city
    pub input: String,
    /// List of cities to display
    pub cities: Vec<String>,
    /// Currently selected city index
    pub selected_city: usize,
    /// Current weather data
    pub current_weather: Option<CityInfo>,
    /// Flag to indicate weather fetch is requested
    pub fetch_requested: bool,
    /// Terminal size
    pub terminal_size: Option<(u16, u16)>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self {
            running: true,
            api_key: String::from("5d916a464e1dced7b9b26a4454d37d40"),
            input_mode: InputMode::Normal,
            input: String::new(),
            cities: vec![
                "Bucharest".to_string(),
                "London".to_string(),
                "New York".to_string(),
                "Budapest".to_string(),
                "Tokyo".to_string(),
                "Paris".to_string(),
                "Berlin".to_string(),
                "Moscow".to_string(),
                "Sydney".to_string(),
                "Toronto".to_string(),
            ],
            selected_city: 0,
            current_weather: None,
            fetch_requested: false,
            terminal_size: None,
        }
    }
    
    /// Returns the ListState for the city list
    pub fn list_state(&mut self) -> ListState {
        let mut state = ListState::default();
        state.select(Some(self.selected_city));
        state
    }
    
    /// Enter edit mode to add a new city
    pub fn enter_edit_mode(&mut self) {
        self.input_mode = InputMode::Editing;
        self.input.clear();
    }
    
    /// Exit edit mode without adding a city
    pub fn exit_edit_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.input.clear();
    }
    
    /// Add the current input as a new city
    pub fn add_city(&mut self) {
        let trimmed = self.input.trim();
        if !trimmed.is_empty() {
            self.cities.push(trimmed.to_string());
            self.selected_city = self.cities.len() - 1;
            // Request weather data for the newly added city
            self.request_weather_fetch();
        }
        self.input_mode = InputMode::Normal;
        self.input.clear();
    }
    
    /// Handle keyboard input when in edit mode
    pub fn handle_input(&mut self, key: char) {
        self.input.push(key);
    }
    
    /// Delete the last character in the input
    pub fn delete_char(&mut self) {
        self.input.pop();
    }
    
    /// Navigate to the next city
    pub fn next_city(&mut self) {
        if !self.cities.is_empty() {
            self.selected_city = (self.selected_city + 1) % self.cities.len();
            // Request weather data for the newly selected city
            self.request_weather_fetch();
        }
    }
    
    /// Navigate to the previous city
    pub fn previous_city(&mut self) {
        if !self.cities.is_empty() {
            self.selected_city = if self.selected_city > 0 {
                self.selected_city - 1
            } else {
                self.cities.len() - 1
            };
            // Request weather data for the newly selected city
            self.request_weather_fetch();
        }
    }
    
    /// Handle terminal resize events
    pub fn handle_resize(&mut self, width: u16, height: u16) {
        self.terminal_size = Some((width, height));
    }
    
    /// Handle tick events
    pub fn tick(&mut self) {
        // Update time-based logic
        // Could implement periodic weather refresh here
    }
    
    /// Fetch weather data for the selected city
    pub async fn fetch_weather(&mut self) -> AppResult<()> {
        if let Some(city) = self.cities.get(self.selected_city).cloned() {
            match crate::connection::get_data(city, &self.api_key).await {
                Ok(weather) => {
                    self.current_weather = Some(weather);
                    Ok(())
                },
                Err(e) => Err(e),
            }
        } else {
            Ok(()) // No city selected, nothing to do
        }
    }
    
    /// Request a weather fetch
    pub fn request_weather_fetch(&mut self) {
        self.fetch_requested = true;
    }
    
    /// Refresh the weather data on demand
    pub fn refresh_weather(&mut self) {
        self.request_weather_fetch();
    }
    
    /// Remove the selected city
    pub fn remove_selected_city(&mut self) {
        if !self.cities.is_empty() {
            self.cities.remove(self.selected_city);
            if self.selected_city >= self.cities.len() && !self.cities.is_empty() {
                self.selected_city = self.cities.len() - 1;
            }
            // Update weather if we still have cities
            if !self.cities.is_empty() {
                self.request_weather_fetch();
            } else {
                self.current_weather = None;
            }
        }
    }
}