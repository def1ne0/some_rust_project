use serde::{Deserialize, Serialize};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let latitude = 53.9;
    let longitude = 27.5667;
    let timezone = "Europe/Moscow";

    let data = get_weather_data(latitude, longitude, timezone).await?;

    println!("Получено {} отсчётов температуры", data.hourly.temperature_2m.len());

    for (i, (time, temp)) in data
        .hourly
        .time
        .iter()
        .zip(data.hourly.temperature_2m.iter())
        .take(5)
        .enumerate()
    {
        println!("{}. {} — {}°C", i + 1, time, temp);
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct ForecastResponse {
    latitude: f64,
    longitude: f64,
    timezone: String,
    hourly: HourlyData,
}

#[derive(Serialize, Deserialize, Debug)]
struct HourlyData {
    time: Vec<String>,
    temperature_2m: Vec<f64>,
}

async fn get_weather_data(
    latitude: f64,
    longitude: f64,
    timezone: &str,
) -> Result<ForecastResponse, Box<dyn Error>> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m&timezone={}",
        latitude, longitude, urlencoding::encode(timezone)
    );

    let resp: ForecastResponse = reqwest::get(&url)
        .await?
        .json()
        .await?;
    Ok(resp)
}