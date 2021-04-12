#[derive(Debug, PartialEq)]
pub struct Foo {
    // Lots of complicated fields.
    bar: String,
}

impl Foo {
    // This method will help users to discover the builder
    pub fn builder() -> FooBuilder {
        FooBuilder::default()
    }
}

#[derive(Default)]
pub struct FooBuilder {
    // Probably lots of optional fields.
    bar: String,
}

impl FooBuilder {
    pub fn new(/* ... */) -> FooBuilder {
        // Set the minimally required fields of Foo.
        FooBuilder {
            bar: String::from("X"),
        }
    }

    pub fn name(mut self, bar: String) -> FooBuilder {
        // Set the name on the builder itself, and return the builder by value.
        self.bar = bar;
        self
    }

    // If we can get away with not consuming the Builder here, that is an
    // advantage. It means we can use the FooBuilder as a template for constructing
    // many Foos.
    pub fn build(self) -> Foo {
        // Create a Foo from the FooBuilder, applying all settings in FooBuilder
        // to Foo.
        Foo { bar: self.bar }
    }
}

// The example takes and returns the builder by value. It is often more ergonomic
// (and more efficient) to take and return the builder as a mutable reference.
// The borrow checker makes this work naturally.
// This approach has the advantage that one can write code like

// let mut fb = FooBuilder::new();
// fb.a();
// fb.b();
// let f = fb.build();

// as well as the FooBuilder::new().a().b().build() style.

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn builder_test() {
        let foo = Foo {
            bar: String::from("Y"),
        };
        let foo_from_builder: Foo = FooBuilder::new().name(String::from("Y")).build();
        assert_eq!(foo, foo_from_builder);
    }
}
