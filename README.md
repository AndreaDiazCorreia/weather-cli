# Weather Station CLI

A command-line interface application built in Rust that provides real-time weather information using the OpenWeatherMap API. This application features colored output, temperature unit conversion, and an interactive command-line interface.

## Features

- Real-time weather data retrieval from OpenWeatherMap
- Support for multiple temperature units (Celsius/Fahrenheit)
- Colored terminal output based on weather conditions
- Weather condition emojis for better visualization
- Interactive command-line interface with command history
- Customizable default settings through configuration file

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust and Cargo (Latest stable version)
- An OpenWeatherMap API key (Get one at: https://openweathermap.org/api)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/weather-station
cd weather-station
```

2. Create your configuration file:
```bash
cp config.tpl.toml config.toml
```

3. Edit `config.toml` and add your OpenWeatherMap API key:
```toml
api_key = "your_api_key_here"
default_country = "US"
temperature_unit = "celsius"  # or "fahrenheit"
```

4. Build the project:
```bash
cargo build --release
```

## Configuration

The application uses a TOML configuration file with the following settings:

- `api_key`: Your OpenWeatherMap API key
- `default_country`: Default country code for weather queries (e.g., "US", "GB", "ES")
- `temperature_unit`: Preferred temperature unit ("celsius" or "fahrenheit")

### Example Configuration

```toml
api_key = "your_api_key_here"
default_country = "US"
temperature_unit = "celsius"
```

## Usage

Run the application using Cargo:

```bash
cargo run --release
```

The application will prompt you for:
1. City name
2. Country code (optional - will use default if left empty)

Example session:
```
⚡ Welcome to Weather Station! ⚡
Enter city name (or 'quit' to exit) > London
Enter country code (press Enter for default) > GB

Weather in London, GB: clear sky 🌤️
> Temperature: 18.5°C (feels like 17.8°C)
> Range: 16.2°C - 20.1°C
> Humidity: 65.0%
> Pressure: 1013.0 hPa
> Wind: 3.6 m/s ↗
```

### Commands

- Enter a city name to get weather information
- Press Enter without a country code to use the default country
- Type 'quit' to exit the application
- Use up/down arrow keys to navigate through command history

## Weather Display

The application shows:
- Current temperature with "feels like" temperature
- Temperature range (min/max)
- Humidity percentage
- Atmospheric pressure
- Wind speed and direction
- Weather condition with appropriate emoji
- Color-coded output based on weather conditions:
  - Yellow: Clear sky
  - Blue: Cloudy conditions
  - Cyan: Rain or snow
  - Purple: Thunderstorms

## Error Handling

The application includes robust error handling for:
- Invalid API keys
- Network connection issues
- Invalid city names or country codes
- Configuration file issues

If you encounter any errors, the application will display helpful error messages with suggestions for resolution.

## Security

The application uses a template configuration file (`config.tpl.toml`) to avoid accidentally committing API keys to version control. Always ensure that:

1. `config.toml` is listed in your `.gitignore`
2. Never commit your actual API key to version control
3. Keep your API key secure and don't share it publicly

## Contributing

Contributions are welcome! Please feel free to submit pull requests with improvements or bug fixes.

When contributing, please:
1. Fork the repository
2. Create a new branch for your feature
3. Add appropriate tests
4. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Weather data provided by OpenWeatherMap
- Built with Rust and various community crates
- Terminal colors provided by the colored crate
- Configuration handling by the config crate

## Support

If you encounter any issues or have questions, please:
1. Check the existing issues in the repository
2. Create a new issue with a detailed description of your problem
3. Include your Rust version and operating system details

