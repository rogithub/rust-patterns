use std::{path::PathBuf, time::Duration};

// note that we can simply auto-derive Default here.
#[derive(Default, Debug)]
struct MyConfiguration {
    // Option defaults to None
    output: Option<PathBuf>,
    // Vecs default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}

impl MyConfiguration {
    // add setters here
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn word() {
        // construct a new instance with default values
        let mut conf = MyConfiguration::default();
        // do something with conf here
        conf.check = true;
        assert_eq!(conf.check, true);
    }
}
