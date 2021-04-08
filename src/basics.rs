pub fn calculate_length_borrowing(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn calculate_length_borrowing_test() {
        let s1 = String::from("hello");
        let (_s2, len) = calculate_length_borrowing(s1);
        // assert_eq!(_s2, s1); // using s1 here causes error, because it was moved int fn!
        assert_eq!(len, 5);
    }
}
