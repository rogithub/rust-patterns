/*
    RAII stands for "Resource Acquisition is Initialisation" which is a terrible name.
    The essence of the pattern is that resource initialisation is done in the constructor of an object and
    finalisation in the destructor.

    This pattern is extended in Rust by using an RAII object as a guard of some resource and relying on the
    type system to ensure that access is always mediated by the guard object.
*/
use std::time::{Duration, Instant};

pub struct Timer<'a> {
    pub name: &'a str,
    pub elapsed: &'a mut Option<Duration>,
    start: Instant,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str, duration: &'a mut Option<Duration>) -> Timer<'a> {
        Timer {
            name,
            start: Instant::now(),
            elapsed: duration,
        }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        *self.elapsed = Some(self.start.elapsed())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_type_testing() {
        let one_thousand: u64 = 1000;
        let mut elapsed: Option<Duration> = None;
        {
            let _timer = Timer::new("testing", &mut elapsed);
            let one_sec = Duration::from_millis(one_thousand);
            std::thread::sleep(one_sec);
        }

        let duration = elapsed.unwrap();

        assert_eq!(duration.as_millis(), one_thousand as u128);
    }
}
