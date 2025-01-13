use reqwest;
use std::error::Error;
use crate::weather::models::WeatherResponse;

pub async fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str
) -> Result<WeatherResponse, Box<dyn Error>> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::get(&url).await?;
    
    if !response.status().is_success() {
        return Err(format!(
            "API request failed: {} ({})",
            response.status(),
            response.text().await?
        ).into());
    }

    let weather_data = response.json::<WeatherResponse>().await?;
    Ok(weather_data)
}

pub async fn get_forecast(
    city: &str,
    country_code: &str,
    api_key: &str
) -> Result<ForecastResponse, Box<dyn Error>> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/forecast?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::get(&url).await?;
    let forecast_data = response.json::<ForecastResponse>().await?;
    Ok(forecast_data)
}