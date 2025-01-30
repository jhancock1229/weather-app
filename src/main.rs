use chrono::DateTime;
use clap::Parser;
use dotenv::dotenv;
use reqwest::blocking::get;
use serde::Deserialize;
use std::env;

#[derive(Parser)]
#[command(name = "weather-cli")]
#[command(version = "1.1")]
#[command(about = "Fetches weather information for a given location.")]
struct Args {
    /// City name (optional, will use geolocation if omitted)
    city: Option<String>,

    /// Temperature unit: metric (°C), imperial (°F), or standard (K)
    #[arg(short, long, default_value = "metric")]
    units: String,
}
#[derive(Deserialize)]
struct WeatherData {
    current: CurrentWeather,
    hourly: Vec<HourlyWeather>, // Hourly forecast data
}

#[derive(Deserialize)]
struct CurrentWeather {
    temp: f64,
    humidity: u8,
    wind_speed: f64,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct HourlyWeather {
    dt: u64, // Timestamp for the hourly forecast
    temp: f64,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

fn get_weather(
    lat: &str,
    lon: &str,
    api_key: &str,
    units: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&appid={}&units={}",
        lat, lon, api_key, units
    );

    let response = get(&url)?.json::<WeatherData>()?;

    let unit_symbol = match units {
        "imperial" => "°F",
        "standard" => "K",
        _ => "°C",
    };

    println!("🌍 Location: {}, {}", lat, lon);
    println!(
        "🌡️ Temperature: {:.1}{}",
        response.current.temp, unit_symbol
    );
    println!("💨 Wind Speed: {:.1} m/s", response.current.wind_speed);
    println!("💧 Humidity: {}%", response.current.humidity);
    println!("🌥️ Condition: {}", response.current.weather[0].description);

    println!("\n📅 Hourly Forecast:");
    for hour in response.hourly.iter().take(5) {
        // Show next 5 hours
        let time = DateTime::from_timestamp(hour.dt as i64, 0)
            .map(|t| t.format("%H:%M").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        println!(
            "🕒 {} - {:.1}{} ({})",
            time, hour.temp, unit_symbol, hour.weather[0].description
        );
    }

    Ok(())
}

fn get_location() -> Result<(String, String), Box<dyn std::error::Error>> {
    let url = "http://ip-api.com/json/";
    let response = get(url)?.json::<serde_json::Value>()?;

    let lat = response["lat"].as_f64().unwrap_or(0.0).to_string();
    let lon = response["lon"].as_f64().unwrap_or(0.0).to_string();

    Ok((lat, lon))
}

fn get_city_coordinates(
    city: &str,
    api_key: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=1&appid={}",
        city, api_key
    );

    let response = get(&url)?.json::<Vec<serde_json::Value>>()?;

    if response.is_empty() {
        return Err("City not found".into());
    }

    let lat = response[0]["lat"].as_f64().unwrap_or(0.0).to_string();
    let lon = response[0]["lon"].as_f64().unwrap_or(0.0).to_string();

    Ok((lat, lon))
}
fn main() {
    dotenv().ok();
    let args = Args::parse();

    let api_key = env::var("OPENWEATHER_API_KEY")
        .expect("API key not found. Set OPENWEATHER_API_KEY in .env");

    let (lat, lon) = match args.city {
        Some(city) => {
            // Convert city to lat/lon using OpenWeatherMap's Geocoding API
            match get_city_coordinates(&city, &api_key) {
                Ok(coords) => coords,
                Err(_) => {
                    eprintln!("Failed to get coordinates for city '{}'", city);
                    return;
                }
            }
        }
        None => match get_location() {
            Ok(coords) => coords,
            Err(_) => {
                eprintln!("Failed to get geolocation.");
                return;
            }
        },
    };

    if let Err(e) = get_weather(&lat, &lon, &api_key, &args.units) {
        eprintln!("Error fetching weather: {}", e);
    }
}
