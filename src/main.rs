//use std::collections::HashMap;
//use reqwest;
//use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use structopt::StructOpt;
use serde::Deserialize;
//use serde_json::Result;
//use chrono::Utc;
use wreport::weather::Weather;

/// Report current weather conditions
#[derive(StructOpt, Debug)]
//#[structopt(name = "wreport")]
struct Opt {
    /// Latitude
    #[structopt(long, required(false), default_value)]
    lat: f64,

    /// Longitude
    #[structopt(long, required(false), default_value)]
    lon: f64,

    /// Config file
    //#[structopt(short, long, parse(from_os_str), required(false), default_value = "~/.wreport/config")]
    //config: PathBuf,

    /// API key
    #[structopt(short, long, required(false), default_value)]
    key: String,
}


#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    lat: f32,
    lon: f32,
}

#[derive(Deserialize, Debug)]
struct Config {
    openweather_api_key: String,
    locations: Vec<Location>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    //let path = shellexpand::tilde(&opt.config)
    let mut path = home::home_dir().unwrap();
    path.push(".wreport");
    path.push("config.yaml");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config: Config = serde_yaml::from_reader(reader)?;

    println!("{:#?}", config);

    let weather = Weather::new(opt.lat, opt.lon, opt.key);
    
    Ok(())
}