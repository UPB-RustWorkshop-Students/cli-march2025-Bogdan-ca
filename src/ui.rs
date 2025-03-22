use ratatui::Frame;
use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout, Alignment, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Clear, Sparkline};
use unicode_width::UnicodeWidthStr;
use crate::app::{App, InputMode};

/// Renders the complete user interface.
pub fn render(app: &mut App, frame: &mut Frame) {
    // Main dashboard block with title and border
    let main_block = Block::default()
        .borders(Borders::ALL)
        .title(" Weather CLI Dashboard ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));
    let inner_area = main_block.inner(frame.size());
    frame.render_widget(main_block, frame.size());

    // Split the dashboard into two vertical sections: Cities and Weather Details.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // cities section
            Constraint::Min(10),    // weather details section
        ].as_ref())
        .split(inner_area);
    
    // --- Cities Block ---
    let cities_block = Block::default()
        .borders(Borders::ALL)
        .title(" Cities ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Magenta));
    let cities_area = cities_block.inner(chunks[0]);
    frame.render_widget(cities_block, chunks[0]);

    let cities: Vec<ListItem> = app.cities
        .iter()
        .enumerate()
        .map(|(i, city)| {
            let is_selected = i == app.selected_city;
            let prefix = if is_selected { "➤ " } else { "  " };
            ListItem::new(format!("{}{}", prefix, city))
                .style(Style::default().fg(if is_selected { Color::Yellow } else { Color::White }))
        })
        .collect();
    let list_component = List::new(cities)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    frame.render_stateful_widget(list_component, cities_area, &mut app.list_state());

    // --- Weather Details Block ---
    let weather_block = Block::default()
        .borders(Borders::ALL)
        .title(" Weather Details ")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Green));
    let weather_area = weather_block.inner(chunks[1]);
    frame.render_widget(weather_block, chunks[1]);

    // Divide the weather area into 70% text and 30% graph
    let weather_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ].as_ref())
        .split(weather_area);

    // Build weather details text (if available)
    let weather_text = if let Some(weather) = &app.current_weather {
        Text::from(vec![
            Line::from(vec![
                Span::styled("City: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}, {}", weather.name, weather.country)),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::styled("Temp: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{:.1}°C (feels like {:.1}°C)", weather.temperature, weather.feels_like)),
            ]),
            Line::from(vec![
                Span::styled("Range: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{:.1}°C - {:.1}°C", weather.temp_min, weather.temp_max)),
            ]),
            Line::from(vec![
                Span::styled("Conditions: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} ({})", weather.weather_main, weather.description)),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::styled("Humidity: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}%", weather.humidity)),
            ]),
            Line::from(vec![
                Span::styled("Wind: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{:.1} m/s", weather.wind_speed)),
            ]),
            Line::from(vec![
                Span::styled("Pressure: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} hPa", weather.pressure)),
            ]),
            Line::raw(""),
            Line::from(vec![
                Span::styled("Sunrise: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(match weather.sunrise {
                    Some(time) => format!("{}", time.format("%H:%M")),
                    None => "N/A".to_string(),
                }),
            ]),
            Line::from(vec![
                Span::styled("Sunset: ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(match weather.sunset {
                    Some(time) => format!("{}", time.format("%H:%M")),
                    None => "N/A".to_string(),
                }),
            ]),
        ])
    } else {
        Text::from(vec![
            Line::raw("No weather data available"),
            Line::raw("Press Enter to fetch weather"),
        ])
    };

    let weather_info = Paragraph::new(weather_text)
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White));
    frame.render_widget(weather_info, weather_chunks[0]);

    // Render sparkline graph (if hourly data exists) in the lower weather area.
    if let Some(weather) = &app.current_weather {
        // Convert f64 temperatures to u64 values for the sparkline.
        let sparkline_data: Vec<u64> = weather.hourly_temps.clone().unwrap_or_default()
            .iter().map(|&temp| temp.round() as u64).collect();
            
        let sparkline = Sparkline::default()
            .block(Block::default().title("Next Hours").borders(Borders::ALL))
            .data(&sparkline_data)
            .style(Style::default().fg(Color::Green))
            .max(40);
        frame.render_widget(sparkline, weather_chunks[1]);
    }

    // Render input popup if in editing mode.
    if app.input_mode == InputMode::Editing {
        render_input_popup(app, frame);
    }
}

/// Renders the input popup for adding a new city.
fn render_input_popup(app: &App, frame: &mut Frame) {
    let area = centered_rect(60, 20, frame.size());
    
    let input_block = Block::default()
        .title(" Add City ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    
    let input_area = input_block.inner(area); // Get inner area before rendering
    frame.render_widget(Clear, area); // Clear the popup area first
    frame.render_widget(input_block, area);
    
    let input_text = Paragraph::new(Text::from(app.input.as_str()))
        .style(Style::default().fg(Color::White));
    frame.render_widget(input_text, input_area);
    
    // Position the cursor within the input area.
    frame.set_cursor(
        input_area.x + UnicodeWidthStr::width(app.input.as_str()) as u16,
        input_area.y,
    );
}

/// Helper to create a centered rectangle with given width and height percentages.
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