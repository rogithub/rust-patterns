pub fn calculate_length_borrowing(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

pub fn calculate_length_without_borrowing(s: &String) -> usize {
    s.len()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calculate_length_borrowing_test() {
        let s1 = String::from("hello");
        let (_s2, len) = calculate_length_borrowing(s1);
        // assert_eq!(_s2, s1); // using s1 here causes error, because it was moved into fn!
        assert_eq!(len, 5);
    }

    #[test]
    fn calculate_length_without_borrowing_test() {
        let s1 = String::from("hello");
        // Passing a ref to the function, so it does not takes ownership of s1 variable
        let len = calculate_length_without_borrowing(&s1);
        // possible to use s1 variable because wasn't borrowed in fn call above.
        assert_eq!(s1, "hello");
        assert_eq!(len, 5);
    }
}
