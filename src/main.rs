use std::env;

use chrono::{NaiveDateTime, DateTime, Utc, format::{DelayedFormat, StrftimeItems}};
use dotenv::dotenv;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}


#[derive(Debug, Deserialize, Serialize)]
struct Forecast {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    visibility: u32,
    wind: Wind,
    clouds: Clouds,
    dt: u64,
    sys: Sys,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Coord {
    lon: f64,
    lat: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    pressure: u32,
    humidity: u32,
    temp_min: f64,
    temp_max: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Clouds {
    all: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Sys {
    r#type: u32,
    id: u32,
    country: String,
    sunrise: u64,
    sunset: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    let resp = Forecast::get(args.city, &args.country_code).await?;
    
    println!(r#"
        location:       {}, {}
        temperature:    {}ºC
        feels like:     {}ºC
        humidity:       {}%
        wind:           {}m/s
        wind direction: {}
        sunrise:        {} UTC
        sunset:         {} UTC
        "#,
        resp.name, resp.sys.country, resp.main.temp, resp.main.feels_like, resp.main.humidity, resp.wind.speed, degree_to_compass(resp.wind.deg), utc_to_time(resp.sys.sunrise), utc_to_time(resp.sys.sunset)
    );
    
     Ok(())
}

fn api_key() -> String {
    dotenv().ok(); // Load environment variables from .env file

     env::var("API_KEY").expect("could not load the API key")
} 

impl Forecast {
    async fn get(city: String, country_code: &String) -> Result<Self, Box<dyn std::error::Error>> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
            city,
            country_code,
            api_key()
        );
        println!("{}", url);
        let url = Url::parse(url.as_str())?;
        
        let resp = reqwest::get(url)
            .await?
            .json::<Forecast>()
            .await?;

        Ok(resp)
    }

}

fn degree_to_compass(deg: u32) -> &'static str {
    match deg {
        0..=22 => "N",
        23..=67 => "NE",
        68..=112 => "E",
        113..=157 => "SE",
        158..=202 => "S",
        203..=247 => "SW",
        248..=292 => "W",
        293..=337 => "NW",
        338..=360 => "N",
        _ => "error getting direction",
    }
}

fn utc_to_time<'a>(timestamp: u64) -> DelayedFormat<StrftimeItems<'a>> {
    let dt = NaiveDateTime::from_timestamp(timestamp as i64, 0);
    let dt = DateTime::<Utc>::from_utc(dt, Utc);
    
    dt.format("%H:%M:%S")
}
