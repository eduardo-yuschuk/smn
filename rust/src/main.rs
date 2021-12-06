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
    pub fn create(days: i32) -> WeatherReport {
        let url = format!("https://ws.smn.gob.ar/map_items/forecast/{}", days);
        let response = reqwest::blocking::get(&url)
            .expect(&format!("Error al acceder al servicio en la URL {}!", url));
        WeatherReport {
            reports: response
                .json::<Vec<Report>>()
                .expect("Error al interpretar la respueta del servicio!"),
        }
    }

    pub fn get_provinces(&self) -> Vec<String> {
        let mut provinces: Vec<String> = self
            .reports
            .as_slice()
            .into_iter()
            .map(|report| report.province.clone())
            .collect();
        provinces.sort();
        provinces.dedup();
        provinces
    }

    pub fn get_cities(&self, province: &str) -> Vec<String> {
        let mut cities: Vec<String> = self
            .reports
            .as_slice()
            .into_iter()
            .filter(|report| report.province.eq_ignore_ascii_case(province))
            .map(|report| report.name.clone())
            .collect();
        cities.sort();
        cities
    }

    pub fn get_all_cities(&self) -> Vec<String> {
        let mut cities: Vec<String> = self
            .reports
            .as_slice()
            .into_iter()
            .map(|report| report.name.clone())
            .collect();
        cities.sort();
        cities
    }

    pub fn get_report(&self, province: &str, city: &str) -> &Report {
        self.reports
            .as_slice()
            .into_iter()
            .filter(|report| {
                report.province.eq_ignore_ascii_case(province)
                    && report.name.eq_ignore_ascii_case(city)
            })
            .collect::<Vec<&Report>>()
            .first()
            .expect("")
    }
}

/// lectura a través de un puntero a función
pub fn get_weather_reports<F>(days: i32, function: F)
where
    F: Fn(&Report),
{
    let url = format!("https://ws.smn.gob.ar/map_items/forecast/{}", days);
    let response = reqwest::blocking::get(&url)
        .expect(&format!("Error al acceder al servicio en la URL {}!", url));
    let reports = response
        .json::<Vec<Report>>()
        .expect("Error al interpretar la respueta del servicio!");
    for report in reports {
        function(&report);
    }
}

fn main() {
    println!("Buscamos el pronóstico a 1, 2 o 3 días?:");
    let weather_report = WeatherReport::create(read_int_from_stdin());

    println!("Seleccione una provincia (ingrese el número):");
    let provinces = weather_report.get_provinces();
    for (id, province) in provinces.iter().enumerate() {
        println!("[{}] {}", id, province);
    }
    let province = provinces
        .get(read_int_from_stdin() as usize)
        .expect("Provincia no encontrada!");

    println!("Seleccione una ciudad:");
    let cities = weather_report.get_cities(province);
    for (id, city) in cities.iter().enumerate() {
        println!("[{}] {}", id, city);
    }
    let city = cities
        .get(read_int_from_stdin() as usize)
        .expect("Ciudad no encontrada!");

    let city_report = weather_report.get_report(province, city);
    println!("{}", city_report.weather);
}

fn read_int_from_stdin() -> i32 {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error al leer el valor");
    let value = buffer
        .trim()
        .parse::<i32>()
        .expect("Error al interpretar el valor");
    value
}
