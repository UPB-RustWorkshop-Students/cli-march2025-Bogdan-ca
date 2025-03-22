use ratatui::Frame;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Clear};
use unicode_width::UnicodeWidthStr; // Add this import for string width
use crate::app::{App, InputMode};

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) { // Remove generic parameter
    // Create the main border for the whole app
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" Weather CLI App ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::White));
    
    // Get the inner area inside the main block
    let inner_area = main_block.inner(frame.size());
    
    // Render the main block
    frame.render_widget(main_block, frame.size());
    
    // Split the inner area into two vertical sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),  // Cities section (fixed height for header + 3 cities)
            Constraint::Min(5),     // Weather details section
        ].as_ref())
        .split(inner_area);
    
    // Create the cities section
    let cities_block = Block::default()
        .borders(Borders::ALL)
        .title(" Cities: ")
        .border_style(Style::default().fg(Color::White));
    
    // Get the inner area of the cities block BEFORE rendering
    let cities_area = cities_block.inner(chunks[0]);
    
    // Render the cities block
    frame.render_widget(cities_block, chunks[0]);
    
    // Create city list items with checkbox style
    let cities: Vec<ListItem> = app.cities
        .iter()
        .enumerate()
        .map(|(i, city)| {
            let is_selected = i == app.selected_city;
            let prefix = if is_selected { "[X] " } else { "[ ] " };
            ListItem::new(format!("{}{}", prefix, city))
                .style(Style::default().fg(if is_selected { Color::Yellow } else { Color::White }))
        })
        .collect();
    
    // Create the list component
    let list_component = List::new(cities)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("");
    
    // Render the list inside the cities block
    frame.render_stateful_widget(list_component, cities_area, &mut app.list_state());
    
    // Create the weather details block
    let weather_block = Block::default()
        .borders(Borders::ALL)
        .title(" Weather Details: ")
        .border_style(Style::default().fg(Color::White));
    
    // Get the inner area BEFORE rendering
    let weather_area = weather_block.inner(chunks[1]);
    
    // Render the weather block
    frame.render_widget(weather_block, chunks[1]);
    
    // Create weather details text
    let weather_text = if let Some(weather) = &app.current_weather {
        Text::from(vec![
            Line::from(vec![
                Span::styled("City: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{}, {}", weather.name, weather.country)),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::styled("Temperature: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.1}°C (feels like {:.1}°C)", weather.temperature, weather.feels_like)),
            ]),
            Line::from(vec![
                Span::styled("Range: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.1}°C - {:.1}°C", weather.temp_min, weather.temp_max)),
            ]),
            Line::from(vec![
                Span::styled("Conditions: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{} ({})", weather.weather_main, weather.description)),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::styled("Humidity: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{}%", weather.humidity)),
            ]),
            Line::from(vec![
                Span::styled("Wind: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{:.1} m/s", weather.wind_speed)),
            ]),
            Line::from(vec![
                Span::styled("Pressure: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{} hPa", weather.pressure)),
            ]),
        ])
    } else {
        Text::from(vec![
            Line::raw("No weather data available"),
            Line::raw("Press Enter to fetch weather"),
        ])
    };
    
    // Create the paragraph widget
    let weather_info = Paragraph::new(weather_text)
        .style(Style::default().fg(Color::White));
    
    // Render the weather details
    frame.render_widget(weather_info, weather_area);
    
    // Render input popup if in edit mode
    if app.input_mode == InputMode::Editing {
        render_input_popup(app, frame);
    }
}

/// Renders the input popup for adding a new city
fn render_input_popup(app: &App, frame: &mut Frame) { // Remove generic parameter
    let area = centered_rect(60, 20, frame.size());
    
    let input_block = Block::default()
        .title(" Add City ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    
    // Get inner area BEFORE rendering - this is the key fix
    let input_area = input_block.inner(area);
    
    frame.render_widget(Clear, area); // Clear the area first
    frame.render_widget(input_block, area); // Remove clone() - we don't need it anymore
    
    let input_text = Paragraph::new(Text::from(app.input.as_str()))
        .style(Style::default());
    
    frame.render_widget(input_text, input_area);
    
    // Show cursor at input position
    frame.set_cursor(
        input_area.x + UnicodeWidthStr::width(app.input.as_str()) as u16,
        input_area.y,
    );
}

/// Helper function to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ].as_ref())
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ].as_ref())
        .split(popup_layout[1])[1]
}