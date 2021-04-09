// new type representing temperature
pub struct Fahrenheit(f64);
// new type representing temperature
pub struct Celsius(f64);

impl Fahrenheit {
    pub fn to_celsius(&self) -> Celsius {
        let c = ((self.0) - 32.0) / 1.8000;
        Celsius(c)
    }
}

impl Celsius {
    pub fn to_fahrenheit(&self) -> Fahrenheit {
        let f = (self.0 * 1.8) + 32.0;
        Fahrenheit(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_type_testing() {
        let c = Celsius(37.0);
        let f = c.to_fahrenheit();
        assert_eq!(f.0, 98.60000000000001);

        let c = f.to_celsius();
        assert_eq!(c.0, 37.00000000000001);
    }
}
