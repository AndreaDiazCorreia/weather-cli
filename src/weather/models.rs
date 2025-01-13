use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub weather: Vec<Weather>,
    pub main: Main,
    pub wind: Wind,
    pub name: String,
    pub sys: System,  
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub description: String,
    pub main: String,  
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,  
    pub humidity: f64,
    pub pressure: f64,
    pub temp_min: f64,    
    pub temp_max: f64,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: Option<f64>,  
}

#[derive(Deserialize, Debug)]
pub struct System {
    pub country: String,
}