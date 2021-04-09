// https://en.wikipedia.org/wiki/Euclidean_distance

#[macro_export]
macro_rules! norm {
    ($($element:expr),*) => {
        {
            let mut n = 0.0;
            $(
                n += ($element as f64)*($element as f64);
            )*
            n.sqrt()
        }
    };
}

#[cfg(test)]
mod test {

    #[test]
    fn interpreter_euclidean_length_tests() {
        let x = -3f64;
        let y = 4f64;

        assert_eq!(3f64, norm!(x));
        assert_eq!(5f64, norm!(x, y));
        assert_eq!(0f64, norm!(0, 0, 0));
        assert_eq!(1f64, norm!(0.5, -0.5, 0.5, -0.5));
    }
}
