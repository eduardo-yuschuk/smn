#[macro_use]
extern crate serde_derive;
use std::fmt;
use std::io;

#[derive(Deserialize, Debug)]
pub struct Report {
    _id: String,
    dist: f32,
    lid: i32,
    fid: i32,
    name: String,
    province: String,
    lat: String,
    lon: String,
    zoom: String,
    updated: i32,
    weather: Weather,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    day: i32,
    morning_temp: i32,
    morning_id: i32,
    morning_desc: String,
    afternoon_temp: i32,
    afternoon_id: i32,
    afternoon_desc: String,
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = "Error escribiendo en consola";
        writeln!(f, "").expect(msg);
        writeln!(f, "Pronóstico para dentro de {} días", self.day).expect(msg);
        writeln!(f, "").expect(msg);
        writeln!(f, "Por la mañana -----").expect(msg);
        writeln!(f, "Temperatura: {}", self.morning_temp).expect(msg);
        writeln!(f, "Condiciones climáticas: {}", self.morning_desc).expect(msg);
        writeln!(f, "").expect(msg);
        writeln!(f, "Por la tarde ------").expect(msg);
        writeln!(f, "Temperatura: {}", self.afternoon_temp).expect(msg);
        writeln!(f, "Condiciones climáticas: {}", self.afternoon_desc).expect(msg);
        Ok(())
    }
}

pub struct WeatherReport {
    reports: Vec<Report>,
}

impl WeatherReport {
    pub fn create() -> WeatherReport {
        let response = reqwest::blocking::get("https://ws.smn.gob.ar/map_items/forecast/1")
            .expect(&format!("Error al acceder al servicio en la URL {}!", url));
        WeatherReport {
            reports: response
                .json::<Vec<Report>>()
                .expect("Error al interpretar la respuesta del servicio!"),
        }
    }
}

fn main() {
    let weather_report = WeatherReport::create();

    let city_report = weather_report.get_report(province, city);
    println!("{}", city_report.weather);
}

