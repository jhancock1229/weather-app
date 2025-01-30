use reqwest::blocking::get;
use serde::Deserialize;
use clap::Parser;
use std::env;
use dotenv::dotenv;

#[derive(Parser)]
#[command(name = "weather-cli")]
#[command(version = "1.0")]
#[command(about = "Fetches weather information for a given city.")]
struct Args {
    /// City name
    city: String,

    /// Temperature unit: metric (°C), imperial (°F), or standard (K)
    #[arg(short, long, default_value = "metric")]
    units: String,
}
#[derive(Deserialize)]
struct WeatherData {
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
}

#[derive(Deserialize)]
struct Weather {
    description: String,
}

fn get_weather(city: &str, api_key: &str, units: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units={}",
        city, api_key, units
    );

    let response = get(&url)?.json::<WeatherData>()?;

    let unit_symbol = match units {
        "imperial" => "°F",
        "standard" => "K",
        _ => "°C", // Default to metric
    };

    println!("🌍 City: {}", city);
    println!("🌡️ Temperature: {:.1}{}", response.main.temp, unit_symbol);
    println!("🌥️ Condition: {}", response.weather[0].description);

    Ok(())
}
fn main() {
    dotenv().ok(); // Load API key from .env file
    let args = Args::parse();

    let api_key = env::var("OPENWEATHER_API_KEY").expect("API key not found. Set OPENWEATHER_API_KEY in .env");

    if let Err(e) = get_weather(&args.city, &api_key, &args.units) {
        eprintln!("Error fetching weather: {}", e);
    }
}