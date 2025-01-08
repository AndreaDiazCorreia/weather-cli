use std::{env, error::Error};
use config::{Config, File};
use serde::Deserialize;
use colored::*;
use rustyline::Editor;

// Configuration Structures
#[derive(Deserialize, Debug)]
struct AppConfig {
    api_key: String,
    default_country: String,
    temperature_unit: String,
}

// Weather API Response Structures - Enhanced from your original version
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
    sys: System,  // Added to get country information
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
    main: String,  // Added to get main weather condition
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    feels_like: f64,  // Added for better user information
    humidity: f64,
    pressure: f64,
    temp_min: f64,    // Added to show temperature range
    temp_max: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
    deg: Option<f64>,  // Added wind direction, Optional because it's not always available
}

#[derive(Deserialize, Debug)]
struct System {
    country: String,
}

// Temperature handling
#[derive(Debug)]
enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureUnit {
    fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "celsius" | "c" => Ok(TemperatureUnit::Celsius),
            "fahrenheit" | "f" => Ok(TemperatureUnit::Fahrenheit),
            _ => Err("Invalid temperature unit".to_string()),
        }
    }
}

// Weather formatting and display
struct WeatherFormatter {
    unit: TemperatureUnit,
}

impl WeatherFormatter {
    fn new(unit: TemperatureUnit) -> Self {
        Self { unit }
    }

    fn format_temperature(&self, celsius: f64) -> f64 {
        match self.unit {
            TemperatureUnit::Celsius => celsius,
            TemperatureUnit::Fahrenheit => (celsius * 9.0/5.0) + 32.0,
        }
    }

    fn get_unit_symbol(&self) -> &str {
        match self.unit {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
        }
    }
}

// CLI Interface handling
struct WeatherCli {
    editor: Editor<()>,
    config: AppConfig,
    formatter: WeatherFormatter,
}

impl WeatherCli {
    fn new(config: AppConfig) -> Result<Self, Box<dyn Error>> {
        let unit = TemperatureUnit::from_str(&config.temperature_unit)
            .unwrap_or(TemperatureUnit::Celsius);
        
        Ok(Self {
            editor: Editor::<()>::new()?,
            config,
            formatter: WeatherFormatter::new(unit),
        })
    }

    fn prompt(&mut self, prompt: &str) -> Result<String, Box<dyn Error>> {
        match self.editor.readline(&format!("{} > ", prompt)) {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(line)
            }
            Err(err) => Err(err.into()),
        }
    }
}

// Configuration loading
fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let config = Config::builder()
        .add_source(File::with_name("config"))
        .build()?;

    let app_config = config.try_deserialize::<AppConfig>()?;
    Ok(app_config)
}

// Weather API interaction
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Enhanced weather display
fn display_weather_info(response: &WeatherResponse, formatter: &WeatherFormatter) {
    let description = &response.weather[0].description;
    let temperature = formatter.format_temperature(response.main.temp);
    let feels_like = formatter.format_temperature(response.main.feels_like);
    let unit = formatter.get_unit_symbol();

    let weather_text = format!(
        "Weather in {}, {}: {} {}
> Temperature: {:.1}{} (feels like {:.1}{}),
> Range: {:.1}{} - {:.1}{},
> Humidity: {:.1}%, 
> Pressure: {:.1} hPa,
> Wind: {:.1} m/s {}",
        response.name,
        response.sys.country,
        description,
        get_temperature_emoji(response.main.temp),
        temperature,
        unit,
        feels_like,
        unit,
        formatter.format_temperature(response.main.temp_min),
        unit,
        formatter.format_temperature(response.main.temp_max),
        unit,
        response.main.humidity,
        response.main.pressure,
        response.wind.speed,
        get_wind_direction(response.wind.deg),
    );

    let weather_text_colored = match response.weather[0].main.as_str() {
        "Clear" => weather_text.bright_yellow(),
        "Clouds" => weather_text.bright_blue(),
        "Rain" | "Drizzle" | "Snow" => weather_text.bright_cyan(),
        "Thunderstorm" => weather_text.bright_purple(),
        _ => weather_text.normal(),
    };

    println!("{}", weather_text_colored);
}

// Enhanced emoji and wind direction helpers
fn get_temperature_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature < 10.0 {
        "☁️"
    } else if temperature < 20.0 {
        "⛅"
    } else if temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn get_wind_direction(degrees: Option<f64>) -> String {
    match degrees {
        Some(deg) => {
            let directions = ["↑", "↗", "→", "↘", "↓", "↙", "←", "↖"];
            let index = (((deg + 22.5) % 360.0) / 45.0) as usize;
            directions[index].to_string()
        },
        None => "".to_string(),
    }
}

// Enhanced main function
fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", "⚡ Welcome to Weather Station! ⚡".bright_yellow());

    // Load configuration with fallback to environment variables
    let config = match load_config() {
        Ok(config) => config,
        Err(_e) => {
            eprintln!("{}", "Failed to load configuration, using defaults...".yellow());
            AppConfig {
                api_key: env::var("WEATHER_API_KEY")
                    .unwrap_or_else(|_| "YOUR_API_KEY".to_string()),
                default_country: "US".to_string(),
                temperature_unit: "celsius".to_string(),
            }
        }
    };

    let mut cli = WeatherCli::new(config)?;

    loop {
        let city = match cli.prompt("Enter city name (or 'quit' to exit)") {
            Ok(city) => city.trim().to_string(),
            Err(_) => break,
        };

        if city.eq_ignore_ascii_case("quit") {
            break;
        }

        let country = match cli.prompt("Enter country code (press Enter for default)") {
            Ok(country) => {
                let country = country.trim().to_string();
                if country.is_empty() {
                    cli.config.default_country.clone()
                } else {
                    country
                }
            },
            Err(_) => break,
        };

        match get_weather_info(&city, &country, &cli.config.api_key) {
            Ok(response) => {
                display_weather_info(&response, &cli.formatter);
            }
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).bright_red());
                continue;
            }
        }
    }

    println!("{}", "👋 Thank you for using Weather Station!".bright_yellow());
    Ok(())
}