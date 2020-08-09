use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Conditions {
    dt: u32,
}

#[derive(Deserialize, Debug)]
struct OneCall {
    lat: f32,
    lon: f32,
    timezone: String,
    timezone_offset: i32,
    current: Conditions,
}

pub struct Weather {
    current: OneCall,
}

impl Weather {
    pub fn new(lat: f64, lon: f64, key: String) -> Result<Self, reqwest::Error> {
        let url  = format!("https://api.openweathermap.org/data/2.5/onecall?lat={}&lon={}&appid={}&units=imperial&exclude=hourly,daily,minutely", lat, lon, key);
        println!("{}", url);
        //let resp = reqwest::blocking::get(&url)?.text()?;
        let current: OneCall = reqwest::blocking::get(&url)?.json()?;
        println!("{:#?}", current);

        Ok(Self {
            current,
        })
    }
}