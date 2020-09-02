mod temperature;

use temperature::TemperConversion;
use temperature::Temperature;

use std::env;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.result_only {
        println!("{:.2}", config.conversion.result());
    } else {
        println!("{}", config.conversion);
    }

    Ok(())
}

pub struct Config {
    conversion: TemperConversion,
    result_only: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // Getting arguments
        let degrees = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a degrees value"),
        };

        let from = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the origin temperature scale"),
        };

        let to = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get the result temperature scale"),
        };

        let result_only = args.any(|arg| arg == "-ro");

        // Parsing the arguments.
        let degrees: f64 = match degrees.parse() {
            Ok(arg) => arg,
            Err(_) => return Err("Couldn't parse the degrees"),
        };

        let from: Temperature = match from.parse() {
            Ok(temper) => temper,
            Err(_) => return Err("Invalid origin temperature scale"),
        };

        let to: Temperature = match to.parse() {
            Ok(temper) => temper,
            Err(_) => return Err("Invalid result temperature scale"),
        };

        Ok(Config {
            conversion: TemperConversion::new(degrees, from, to),
            result_only,
        })
    }

    pub fn usage(prog_name: &str) {
        eprintln!(
            "Usage: {} <degrees> (origin scale) (result scale) [options]\n\
                   \n\
                   Temperature scales:\n\
                   Celsius, 'c' for short.\n\
                   Fahrenheit, 'f' for short.\n\
                   Kelvin, 'k' for short.\n\
                   Rankine, 'r' for short.\n\
                   \n\
                   Options:\n\
                   -ro: Result Only.\n",
            prog_name
        );
    }
}
