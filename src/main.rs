use ratatui_templates::app::{App, AppResult};
use ratatui_templates::event::{Event, EventsPublisher};
use ratatui_templates::handler::handle_key_events;
use ratatui_templates::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Setup the terminal
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;


    // TODO: create the events pubisher
    let mut tick_rate = 100;
    let mut events_publisher= EventsPublisher::new(tick_rate);

    // TODO: init the terminal user interface
    let mut tui = Tui::new(terminal, events_publisher);
    tui.init()?;
    // Start the main loop.

    app.request_weather_fetch();
    match app.fetch_weather().await {
        Ok(_) => {},
        Err(e) => eprintln!("Error fetching initial weather: {:?}", e),
    }
    app.fetch_requested = false;

    while app.running {
        // TODO: Render the user interface.
        tui.draw(&mut app).expect("failed to draw the user interface");
        // TODO: Handle events.
        // Hint: wait for events and handle them
        match tui.events.next().await {
            Ok(event) => {
                match event {
                    Event::Key(key) => {
                        handle_key_events(key, &mut app);
                    }
                    Event::Mouse(_mouse) => {
                        // We don't have a separate mouse handler
                        // You could handle mouse events here directly or ignore them
                        // Alternatively, you could pass a KeyEvent equivalent
                        // handle_key_events(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE), &mut app);
                    }
                    Event::Resize(width, height) => {
                        // Optional: handle resize events if needed
                        app.handle_resize(width, height);
                    }
                    Event::Tick => {
                        // Update any time-based logic
                        app.tick();
                    }
                }
            },
            Err(e) => {
                eprintln!("Error receiving event: {:?}", e);
            }
        }
        if app.fetch_requested {
            match app.fetch_weather().await {
                Ok(_) => {},
                Err(e) => eprintln!("Error fetching weather: {:?}", e),
            }
            app.fetch_requested = false;
        }
    }

    // TODO: Reset the terminal if the app has been terminated
    tui.exit()?;
    Ok(())
}
