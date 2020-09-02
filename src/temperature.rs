use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

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
            return Ok(Temperature::Celsius);
        }
        if "fahrenheit".starts_with(&s) {
            return Ok(Temperature::Fahrenheit);
        }
        if "kelvin".starts_with(&s) {
            return Ok(Temperature::Kelvin);
        }
        if "rankine".starts_with(&s) {
            return Ok(Temperature::Rankine);
        }
        Err("not implemented")
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Temperature::Celsius => write!(f, "celsius"),
            Temperature::Fahrenheit => write!(f, "fahrenheit"),
            Temperature::Kelvin => write!(f, "kelvin"),
            Temperature::Rankine => write!(f, "rankine"),
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
            Temperature::Celsius => self.convert_celsius(),
            Temperature::Fahrenheit => self.convert_fahrenheit(),
            Temperature::Kelvin => self.convert_kelvin(),
            Temperature::Rankine => self.convert_rankine(),
        }
    }

    fn convert_celsius(&self) -> f64 {
        match self.to {
            Temperature::Celsius => self.degrees,
            Temperature::Fahrenheit => self.degrees * (9.0 / 5.0) + 32.0,
            Temperature::Kelvin => self.degrees + 273.15,
            Temperature::Rankine => self.degrees * (9.0 / 5.0) + 491.67,
        }
    }

    fn convert_fahrenheit(&self) -> f64 {
        match self.to {
            Temperature::Celsius => (self.degrees - 32.0) * (5.0 / 9.0),
            Temperature::Fahrenheit => self.degrees,
            Temperature::Kelvin => (self.degrees - 32.0) * (5.0 / 9.0) + 273.15,
            Temperature::Rankine => self.degrees + 459.67,
        }
    }

    fn convert_kelvin(&self) -> f64 {
        match self.to {
            Temperature::Celsius => self.degrees - 273.15,
            Temperature::Fahrenheit => (self.degrees - 273.15) * (9.0 / 5.0) + 32.0,
            Temperature::Kelvin => self.degrees,
            Temperature::Rankine => (self.degrees - 273.15) * (9.0 / 5.0) + 491.67,
        }
    }

    fn convert_rankine(&self) -> f64 {
        match self.to {
            Temperature::Celsius => (self.degrees - 491.67) * (5.0 / 9.0),
            Temperature::Fahrenheit => self.degrees - 459.67,
            Temperature::Kelvin => (self.degrees - 491.67) * (5.0 / 9.0) + 273.15,
            Temperature::Rankine => self.degrees,
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
        assert_eq!(celsius, Temperature::Celsius);
        let fahrenheit: Temperature = "f".parse().unwrap();
        assert_eq!(fahrenheit, Temperature::Fahrenheit);
        let kelvin: Result<Temperature, _> = "elvin".parse();
        assert!(kelvin.is_err());
        let rankine: Temperature = "RANKINE".parse().unwrap();
        assert_eq!(rankine, Temperature::Rankine);
    }

    #[test]
    fn display_temperature() {
        let celsius = format!("{}", Temperature::Celsius);
        assert_eq!(celsius, "celsius");
        let fahrenheit = format!("{}", Temperature::Fahrenheit);
        assert_eq!(fahrenheit, "fahrenheit");
        let kelvin = format!("{}", Temperature::Kelvin);
        assert_eq!(kelvin, "kelvin");
        let rankine = format!("{}", Temperature::Rankine);
        assert_eq!(rankine, "rankine");
    }

    #[test]
    fn celsius_conversion() {
        let conversion = TemperConversion::new(50.0, Temperature::Celsius, Temperature::Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion = TemperConversion::new(50.0, Temperature::Celsius, Temperature::Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(50.0, Temperature::Celsius, Temperature::Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion = TemperConversion::new(50.0, Temperature::Celsius, Temperature::Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }

    #[test]
    fn fahrenheit_conversion() {
        let conversion =
            TemperConversion::new(122.0, Temperature::Fahrenheit, Temperature::Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion =
            TemperConversion::new(122.0, Temperature::Fahrenheit, Temperature::Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(122.0, Temperature::Fahrenheit, Temperature::Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion =
            TemperConversion::new(122.0, Temperature::Fahrenheit, Temperature::Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }
    #[test]
    fn kelvin_conversion() {
        let conversion = TemperConversion::new(323.15, Temperature::Kelvin, Temperature::Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion =
            TemperConversion::new(323.15, Temperature::Kelvin, Temperature::Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(323.15, Temperature::Kelvin, Temperature::Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion = TemperConversion::new(323.15, Temperature::Kelvin, Temperature::Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }

    #[test]
    fn rankine_conversion() {
        let conversion = TemperConversion::new(581.67, Temperature::Rankine, Temperature::Celsius);
        assert!(float_cmp(conversion.result(), 50.0));
        let conversion =
            TemperConversion::new(581.67, Temperature::Rankine, Temperature::Fahrenheit);
        assert!(float_cmp(conversion.result(), 122.0));
        let conversion = TemperConversion::new(581.67, Temperature::Rankine, Temperature::Kelvin);
        assert!(float_cmp(conversion.result(), 323.15));
        let conversion = TemperConversion::new(581.67, Temperature::Rankine, Temperature::Rankine);
        assert!(float_cmp(conversion.result(), 581.67));
    }

    fn float_cmp(left: f64, right: f64) -> bool {
        let diff = (left - right).abs();
        let epsilon = 0.001;
        diff < epsilon
    }
}
