use crate::weather::models::WeatherResponse;

pub struct WeatherFormatter {
    pub temperature_unit: TemperatureUnit,
    pub wind_speed_unit: WindSpeedUnit,
}

#[derive(Debug, Clone, Copy)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, Clone, Copy)]
pub enum WindSpeedUnit {
    MetersPerSecond,
    KilometersPerHour,
    MilesPerHour,
}

impl WeatherFormatter {
    pub fn new(temperature_unit: TemperatureUnit, wind_speed_unit: WindSpeedUnit) -> Self {
        Self {
            temperature_unit,
            wind_speed_unit,
        }
    }

    pub fn format_temperature(&self, celsius: f64) -> f64 {
        match self.temperature_unit {
            TemperatureUnit::Celsius => celsius,
            TemperatureUnit::Fahrenheit => (celsius * 9.0/5.0) + 32.0,
        }
    }

    pub fn format_wind_speed(&self, meters_per_second: f64) -> f64 {
        match self.wind_speed_unit {
            WindSpeedUnit::MetersPerSecond => meters_per_second,
            WindSpeedUnit::KilometersPerHour => meters_per_second * 3.6,
            WindSpeedUnit::MilesPerHour => meters_per_second * 2.237,
        }
    }

    pub fn get_unit_symbol(&self) -> &'static str {
        match self.temperature_unit {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
        }
    }
}