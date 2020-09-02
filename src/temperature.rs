use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

use Temperature::*;

#[derive(Debug, PartialEq)]
pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,
}

impl FromStr for Temperature {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if "celsius".starts_with(&s) {
            return Ok(Celsius);
        }
        if "fahrenheit".starts_with(&s) {
            return Ok(Fahrenheit);
        }
        if "kelvin".starts_with(&s) {
            return Ok(Kelvin);
        }
        if "rankine".starts_with(&s) {
            return Ok(Rankine);
        }
        Err("not implemented")
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Celsius => write!(f, "celsius"),
            Fahrenheit => write!(f, "fahrenheit"),
            Kelvin => write!(f, "kelvin"),
            Rankine => write!(f, "rankine"),
        }
    }
}

#[derive(Debug)]
pub struct TemperConversion {
    pub degrees: f64,
    pub from: Temperature,
    pub to: Temperature,
    result: f64,
}

impl TemperConversion {
    pub fn new(degrees: f64, from: Temperature, to: Temperature) -> TemperConversion {
        let mut instance = TemperConversion {
            degrees,
            from,
            to,
            result: 0.0,
        };
        instance.result = instance.convert();
        instance
    }

    pub fn result(&self) -> f64 {
        self.result
    }

    fn convert(&self) -> f64 {
        match self.from {
            Celsius => self.convert_celsius(),
            Fahrenheit => self.convert_fahrenheit(),
            Kelvin => self.convert_kelvin(),
            Rankine => self.convert_rankine(),
        }
    }

    fn convert_celsius(&self) -> f64 {
        match self.to {
            Celsius => self.degrees,
            Fahrenheit => self.degrees * (9.0 / 5.0) + 32.0,
            Kelvin => self.degrees + 273.15,
            Rankine => self.degrees * (9.0 / 5.0) + 491.67,
        }
    }

    fn convert_fahrenheit(&self) -> f64 {
        match self.to {
            Celsius => (self.degrees - 32.0) * (5.0 / 9.0),
            Fahrenheit => self.degrees,
            Kelvin => (self.degrees - 32.0) * (5.0 / 9.0) + 273.15,
            Rankine => self.degrees + 459.67,
        }
    }

    fn convert_kelvin(&self) -> f64 {
        match self.to {
            Celsius => self.degrees - 273.15,
            Fahrenheit => (self.degrees - 273.15) * (9.0 / 5.0) + 32.0,
            Kelvin => self.degrees,
            Rankine => (self.degrees - 273.15) * (9.0 / 5.0) + 491.67,
        }
    }

    fn convert_rankine(&self) -> f64 {
        match self.to {
            Celsius => (self.degrees - 491.67) * (5.0 / 9.0),
            Fahrenheit => self.degrees - 459.67,
            Kelvin => (self.degrees - 491.67) * (5.0 / 9.0) + 273.15,
            Rankine => self.degrees,
        }
    }
}

impl Display for TemperConversion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.2} {} is {:.2} {}",
            self.degrees, self.from, self.result, self.to
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_temperature() {
        let celsius: Temperature = "Celsiu".parse().unwrap();
        assert_eq!(celsius, Celsius);
        let fahrenheit: Temperature = "f".parse().unwrap();
        assert_eq!(fahrenheit, Fahrenheit);
        let kelvin: Result<Temperature, _> = "elvin".parse();
        assert!(kelvin.is_err());
        let rankine: Temperature = "RANKINE".parse().unwrap();
        assert_eq!(rankine, Rankine);
    }

    #[test]
    fn display_temperature() {
        let celsius = format!("{}", Celsius);
        assert_eq!(celsius, "celsius");
        let fahrenheit = format!("{}", Fahrenheit);
        assert_eq!(fahrenheit, "fahrenheit");
        let kelvin = format!("{}", Kelvin);
        assert_eq!(kelvin, "kelvin");
        let rankine = format!("{}", Rankine);
        assert_eq!(rankine, "rankine");
    }

    #[test]
    fn celsius_conversion() {
        let conversion = TemperConversion::new(50.0, Celsius, Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion = TemperConversion::new(50.0, Celsius, Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(50.0, Celsius, Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion = TemperConversion::new(50.0, Celsius, Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }

    #[test]
    fn fahrenheit_conversion() {
        let conversion = TemperConversion::new(122.0, Fahrenheit, Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion = TemperConversion::new(122.0, Fahrenheit, Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(122.0, Fahrenheit, Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion = TemperConversion::new(122.0, Fahrenheit, Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }
    #[test]
    fn kelvin_conversion() {
        let conversion = TemperConversion::new(323.15, Kelvin, Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion = TemperConversion::new(323.15, Kelvin, Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(323.15, Kelvin, Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion = TemperConversion::new(323.15, Kelvin, Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }

    #[test]
    fn rankine_conversion() {
        let conversion = TemperConversion::new(581.67, Rankine, Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion = TemperConversion::new(581.67, Rankine, Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(581.67, Rankine, Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion = TemperConversion::new(581.67, Rankine, Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }

    fn float_cmp(left: f64, right: f64) -> bool {
        let diff = (left - right).abs();
        let epsilon = 0.001;
        diff < epsilon
    }
}
