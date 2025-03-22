use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::app::AppResult;

/// Weather information for a city
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityInfo {
    // Basic city information
    pub name: String,
    pub country: String,
    
    // Current weather 
    pub temperature: f64,      // Current temperature in celsius
    pub feels_like: f64,       // "Feels like" temperature
    pub temp_min: f64,         // Minimum temperature
    pub temp_max: f64,         // Maximum temperature
    
    // Weather description
    pub weather_main: String,  // Short description (e.g., "Clear", "Rain")
    pub description: String,   // Detailed description
    pub icon: String,          // Icon ID for weather condition
    
    // Additional data
    pub humidity: u8,          // Humidity percentage
    pub pressure: u32,         // Atmospheric pressure in hPa
    pub wind_speed: f64,       // Wind speed
    pub wind_direction: u16,   // Wind direction in degrees
    
    // Visibility and clouds
    pub visibility: u32,       // Visibility in meters
    pub clouds: u8,            // Cloudiness percentage
    
    // Timestamps
    pub timestamp: DateTime<Utc>, // Time of data calculation
}

/// Fetches weather details from OpenWeather API for the specified city
///
/// Returns weather details about a certain city or an error
pub async fn get_data(city: String, api_key: &str) -> AppResult<CityInfo> {
    // Construct the API URL with your API key
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );
    
    // Make the request (using the async version)
    let response = reqwest::get(&url).await?;
    
    // Check status code
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API error ({}): {}", status, error_text).into());
    }
    
    // Parse the response body into JSON
    let weather_data: serde_json::Value = response.json().await?;
    
    // Extract the needed fields from JSON response
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
        
        timestamp: DateTime::from_timestamp(
            weather_data["dt"].as_i64().unwrap_or(0), 
            0
        ).unwrap_or_else(|| Utc::now()),
    };
    
    Ok(city_info)
}

/// Get the OpenWeather icon URL
pub fn get_icon_url(icon_id: &str) -> String {
    format!("https://openweathermap.org/img/wn/{}@2x.png", icon_id)
}