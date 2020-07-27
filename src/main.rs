//use std::collections::HashMap;
//use reqwest;
use std::path::PathBuf;
use structopt::StructOpt;

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
    #[structopt(short, long, parse(from_os_str), required(false), default_value = "~/.wreport/config")]
    config: PathBuf,

    /// API key
    #[structopt(short, long, required(false), default_value)]
    key: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
    let url  = format!("https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&appid={}", opt.lat, opt.lon, opt.key);
    println!("{}", url);
    let resp = reqwest::blocking::get(&url)?
        .text()?;
    println!("{:#?}", resp);
    Ok(())
}