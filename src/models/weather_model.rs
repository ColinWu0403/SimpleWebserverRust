// use reqwest::Error;
// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// pub struct Forecast {
//     pub properties: Properties,
// }

// #[derive(Debug, Deserialize)]
// pub struct Properties {
//     pub periods: Vec<Period>,
// }

// #[derive(Debug, Deserialize)]
// pub struct Period {
//     pub name: String,
//     pub temperature: i32,
//     pub temperatureUnit: String,
//     pub detailedForecast: String,
// }

// pub async fn get_forecast(lat: f64, lon: f64) -> Result<Properties, Error> {
//     let url = format!("https://api.weather.gov/points/{},{}", lat, lon);

//     // Fetch the initial location data to get the forecast URL
//     let response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;

//     // Extract the forecast URL
//     let forecast_url = response["properties"]["forecast"]
//         .as_str()
//         .ok_or_else(|| {
//             reqwest::Error::new(
//                 reqwest::StatusCode::INTERNAL_SERVER_ERROR,
//                 "Forecast URL not found",
//             )
//         })?;

//     // Fetch the forecast data
//     let forecast_response = reqwest::get(forecast_url).await?;
//     let forecast = forecast_response.json::<Forecast>().await?;

//     Ok(forecast.properties)
// }
