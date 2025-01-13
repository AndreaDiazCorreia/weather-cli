use rustyline::Editor;
use colored::*;
use crate::config::AppConfig;
use crate::weather::api::get_weather_info;
use std::error::Error;

pub enum MenuOpction {
    CurrentWeather,
    Forecats,
    Favorites,
    Settings,
    Exit,
}

pub struct WeatherCli {
    editor: Editor<()>,
    config: AppConfig,
    history: Vec<String>,
}

impl WeatherCli {
    pub fn new(config: AppConfig) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            editor: Editor::<()>::new()?,
            config,
            history: Vec::new(),
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.display_main_menu()? {
                MenuOption::CurrentWeather => self.handle_current_weather()?,
                MenuOption::Forecast => self.handle_forecast()?,
                MenuOption::Favorites => self.handle_favorites()?,
                MenuOption::Settings => self.handle_settings()?,
                MenuOption::Exit => break,
            }
        }
        Ok(())
    }

    fn display_main_menu(&mut self) -> Result<MenuOption, Box<dyn Error>> {
        println!("\n{}", "Weather Station - Main Menu".bright_blue());
        println!("1. Check Current Weather");
        println!("2. View Weather Forecast");
        println!("3. Manage Favorite Locations");
        println!("4. Settings");
        println!("5. Exit");

        loop {
            let input = self.prompt("Select an option (1-5)")?;
            match input.trim() {
                "1" => return Ok(MenuOption::CurrentWeather),
                "2" => return Ok(MenuOption::Forecast),
                "3" => return Ok(MenuOption::Favorites),
                "4" => return Ok(MenuOption::Settings),
                "5" => return Ok(MenuOption::Exit),
                _ => println!("{}", "Invalid option. Please try again.".bright_red()),
            }
        }
    }

    fn handle_current_weather(&mut self) -> Result<(), Box<dyn Error>> {
        let city = self.prompt("Enter city name")?;
        let country = self.prompt("Enter country code (press Enter for default)")?;
        
        let country_code = if country.trim().is_empty() {
            &self.config.default_country
        } else {
            country.trim()
        };

        match get_weather_info(&city, country_code, &self.config.api_key) {
            Ok(weather) => {
                self.history.push(format!("{}, {}", city, country_code));
                crate::cli::display::show_weather(&weather);
            }
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).bright_red());
            }
        }

        Ok(())
    }

    fn prompt(&mut self, message: &str) -> Result<String, Box<dyn Error>> {
        match self.editor.readline(&format!("{} > ", message)) {
            Ok(line) => {
                self.editor.add_history_entry(line.as_str());
                Ok(line)
            }
            Err(err) => Err(err.into()),
        }
    }
}
