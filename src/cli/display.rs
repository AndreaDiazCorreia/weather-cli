use colored::*;
use crate::weather::WeatherResponse;

pub fn show_weather(weather: &WeatherResponse) {
    let description = &weather.weather[0].description;
    let temperature = weather.main.temp;
    let humidity = weather.main.humidity;
    let pressure = weather.main.pressure;
    let wind_speed = weather.wind.speed;

    let weather_text = format!(
        "\nWeather in {}: {} {}
        > Temperature: {:.1}°C
        > Humidity: {:.1}%
        > Pressure: {:.1} hPa
        > Wind Speed: {:.1} m/s",
        weather.name,
        description,
        get_weather_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    println!("{}", weather_text.bright_green());

}

fn get_weather_emoji(temperature: f64) -> &'static str {
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