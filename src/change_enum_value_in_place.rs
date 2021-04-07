use std::fmt;
use std::mem;

pub enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

impl core::fmt::Debug for MyEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyEnum::A { name, x } => f
                .debug_struct("A")
                .field("name", name)
                .field("x", x)
                .finish(),
            MyEnum::B { name } => f.debug_struct("B").field("name", name).finish(),
        }
    }
}

pub fn a_to_b(e: &mut MyEnum) {
    // we mutably borrow `e` here. This precludes us from changing it directly
    // as in `*e = ...`, because the borrow checker won't allow it. Therefore
    // the assignment to `e` must be outside the `if let` clause.
    *e = if let MyEnum::A { ref mut name, x: 0 } = *e {
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`, because it is the result of the `if let` expression).
        MyEnum::B {
            name: mem::take(name),
        }

    // In all other cases, we return immediately, thus skipping the assignment
    } else {
        return;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn a_to_b_test() {
        let name = "Rodrigo".to_string();
        let mut a = MyEnum::A { name: name, x: 0 };
        assert_eq!(format!("{:?}", a), "A { name: \"Rodrigo\", x: 0 }");

        a_to_b(&mut a);

        assert_eq!(format!("{:?}", a), "B { name: \"Rodrigo\" }");
    }
}
