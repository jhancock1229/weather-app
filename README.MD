# Weather CLI

This is a simple command-line application that fetches weather information for a given city using the OpenWeatherMap API.

## Prerequisites

- Rust (https://www.rust-lang.org/tools/install)
- OpenWeatherMap API key (https://home.openweathermap.org/users/sign_up)

## Setup

1. Clone the repository:

   ```sh
   git clone <repository-url>
   cd <repository-directory>
   ```

2. Create a [.env](http://_vscodecontentref_/0) file in the root directory of the project and add your OpenWeatherMap API key:

   ```sh
   echo "OPENWEATHER_API_KEY=your_api_key_here" > .env
   ```

## Running the Application

1. Build the project:

   ```sh
   cargo build --release
   ```

2. Run the application:

   ```sh
   cargo run --release -- --city <city_name> --units <units>
   ```

   - `--city` (required): The name of the city you want to fetch the weather for.
   - `--units` (optional): The temperature unit (`metric` for °C, `imperial` for °F, or `standard` for K). Default is `metric`.

   Example:

   ```sh
   cargo run --release -- --city London --units metric
   ```

## Example Output

🌍 City: London 🌡️ Temperature: 15.0°C 🌥️ Condition: clear sky
