pub fn calculate_length_borrowing(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

pub fn calculate_length_without_borrowing(s: &str) -> usize {
    s.len()
}

pub fn change(s: &mut String) {
    s.push_str(", world!");
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

    #[test]
    fn referencia_mutable_test() {
        let mut s1 = String::from("hello");
        // Passing a ref mut to the function, so it does not takes ownership of s1 variable
        // and also can change it internally
        change(&mut s1);
        // possible to use s1 variable because wasn't borrowed in fn call above.
        assert_eq!(s1, "hello, world!");
    }

    #[test]
    fn one_mut_ref_max_per_scopre() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        //let _r3 = &mut s; // cannot borrow s as mutable (here) because is already borrowed as immutable.
        assert_eq!(r1, "hello");
        assert_eq!(r2, "hello");
        // r1 and r2 are not longer used after this point (moved);
        // so it is ok (no error) to create a mut ref bellow
        let _r3 = &mut s;
        s.push_str(", world!");
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn deref_test_following_ptr_to_the_value() {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn box_implementing_deref_trait() {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
