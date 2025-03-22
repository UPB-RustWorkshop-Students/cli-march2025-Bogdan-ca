use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use crate::app::AppResult;

/// Detailed weather information for a city, including extra data for graphs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityInfo {
    // Basic city information
    pub name: String,
    pub country: String,
    
    // Current weather
    pub temperature: f64,      // current temperature in Celsius
    pub feels_like: f64,       // "feels like" temperature
    pub temp_min: f64,         // minimum temperature
    pub temp_max: f64,         // maximum temperature
    
    // Weather description
    pub weather_main: String,  // Short description (e.g., "Clear", "Rain")
    pub description: String,   // Detailed description
    pub icon: String,          // Icon ID for weather condition
    
    // Additional data
    pub humidity: u8,          // Humidity percentage
    pub pressure: u32,         // Atmospheric pressure in hPa
    pub wind_speed: f64,       // Wind speed in m/s
    pub wind_direction: u16,   // Wind direction in degrees
    pub visibility: u32,       // Visibility in meters
    pub clouds: u8,            // Cloudiness in percentages
    
    // Extra detailed data for graphs
    pub sunrise: Option<DateTime<Utc>>,  // Sunrise time (UTC)
    pub sunset: Option<DateTime<Utc>>,   // Sunset time (UTC)
    
    // Placeholder for hourly temperature data (for drawing graphs)
    pub hourly_temps: Option<Vec<f64>>,
    
    // Timestamp when the data was calculated
    pub timestamp: DateTime<Utc>,
}

/// Fetches weather details from the OpenWeather API for the specified city.
/// Note: To get proper graph data (e.g. hourly temps) you may want to use a different endpoint.
pub async fn get_data(city: String, api_key: &str) -> AppResult<CityInfo> {
    // Construct the API URL (using the "weather" endpoint for current weather)
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    
    // Make the asynchronous request
    let response = reqwest::get(&url).await?;
    
    // Check that the status is OK
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API error ({}): {}", status, error_text).into());
    }
    
    // Parse the response into JSON
    let weather_data: serde_json::Value = response.json().await?;
    
    let city_info = CityInfo {
        name: weather_data["name"].as_str().unwrap_or("Unknown").to_string(),
        country: weather_data["sys"]["country"].as_str().unwrap_or("--").to_string(),
        
        temperature: weather_data["main"]["temp"].as_f64().unwrap_or(0.0),
        feels_like: weather_data["main"]["feels_like"].as_f64().unwrap_or(0.0),
        temp_min: weather_data["main"]["temp_min"].as_f64().unwrap_or(0.0),
        temp_max: weather_data["main"]["temp_max"].as_f64().unwrap_or(0.0),
        
        weather_main: weather_data["weather"][0]["main"].as_str().unwrap_or("Unknown").to_string(),
        description: weather_data["weather"][0]["description"].as_str().unwrap_or("Unknown").to_string(),
        icon: weather_data["weather"][0]["icon"].as_str().unwrap_or("").to_string(),
        
        humidity: weather_data["main"]["humidity"].as_u64().unwrap_or(0) as u8,
        pressure: weather_data["main"]["pressure"].as_u64().unwrap_or(0) as u32,
        wind_speed: weather_data["wind"]["speed"].as_f64().unwrap_or(0.0),
        wind_direction: weather_data["wind"]["deg"].as_u64().unwrap_or(0) as u16,
        
        visibility: weather_data["visibility"].as_u64().unwrap_or(0) as u32,
        clouds: weather_data["clouds"]["all"].as_u64().unwrap_or(0) as u8,
        
        // Extra fields for more detailed data
        sunrise: weather_data["sys"]["sunrise"].as_i64().map(|ts| Utc.timestamp(ts, 0)),
sunset: weather_data["sys"]["sunset"].as_i64().map(|ts| Utc.timestamp(ts, 0)),
        hourly_temps: None, // Placeholder; switch to a forecast endpoint to populate this
        
        timestamp: DateTime::from_timestamp(
            weather_data["dt"].as_i64().unwrap_or(0), 
            0
        ).unwrap_or_else(|| Utc::now()),
    };
    
    Ok(city_info)
}

/// Returns the URL for the weather condition icon.
pub fn get_icon_url(icon_id: &str) -> String {
    format!("https://openweathermap.org/img/wn/{}@2x.png", icon_id)
}