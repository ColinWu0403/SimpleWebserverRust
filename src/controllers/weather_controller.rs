use axum::{
  extract::Query,
  response::IntoResponse,
  http::StatusCode,
  Json,
};
use reqwest;
use serde::{ Deserialize, Serialize };
use anyhow::Result;

#[derive(Deserialize)]
pub struct WeatherQuery {
  lat: f64,
  lon: f64,
}

// The structure for parsing the forecast JSON
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastPeriod {
  pub name: String,
  pub temperature: i32,
  pub temperature_unit: String,
  pub detailed_forecast: String,
}

#[derive(Debug, Deserialize)]
struct ForecastResponse {
  properties: ForecastProperties,
}

#[derive(Debug, Deserialize)]
struct ForecastProperties {
  periods: Vec<ForecastPeriod>,
}

// Fetch the forecast from the NWS API
async fn fetch_forecast(lat: f64, lon: f64) -> Result<Vec<ForecastPeriod>, anyhow::Error> {
  // Step 1: Fetch the location data to get the forecast URL
  let points_url = format!("https://api.weather.gov/points/{},{}", lat, lon);
  let response = reqwest::get(&points_url).await?;
  let body = response.text().await?; // Get the raw response body as text
  
  // Log the body for debugging
  println!("Points API Response: {}", body);

  // Deserialize the response
  let json: serde_json::Value = serde_json::from_str(&body)?;
  let forecast_url = json["properties"]["forecast"]
      .as_str()
      .ok_or_else(|| anyhow::anyhow!("Invalid location"))?;

  // Step 2: Fetch the forecast data from the forecast URL
  let forecast_response = reqwest::get(forecast_url).await?;
  let forecast_body = forecast_response.text().await?;
  
  // Log the forecast body for debugging
  println!("Forecast API Response: {}", forecast_body);

  // Deserialize the forecast response
  let forecast_data: ForecastResponse = serde_json::from_str(&forecast_body)?;

  // Step 3: Return the list of forecast periods
  Ok(forecast_data.properties.periods)
}

// Controller function
pub async fn weather_handler(Query(query): Query<WeatherQuery>) -> impl IntoResponse {
  match fetch_forecast(query.lat, query.lon).await {
      Ok(forecast) => Json(forecast).into_response(),
      Err(err) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          format!("Failed to fetch weather: {}", err),
      )
          .into_response(),
  }
}
