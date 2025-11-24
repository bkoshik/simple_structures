fn main() {}

#[cfg(test)]
mod tests {
    use fast_struct::*;

    #[test]
    fn auto_getters_test() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(AutoGetters)]
        struct Foo {
            bar: String,
            baz: i16,
        }

        let foo = Foo {
            bar: "Hello, world!".into(),
            baz: 1984,
        };

        assert_eq!("Hello, world!", foo.bar());
        assert_eq!(1984_i16, *foo.baz());

        Ok(())
    }

    #[test]
    fn optional_test() -> Result<(), Box<dyn std::error::Error>> {
        #[optional]
        struct Foo {
            bar: String,
            baz: usize,
        }

        let foo = Foo {
            bar: Some("Hello, world!".to_string()),
            baz: Some(1984),
        };

        assert_eq!(Some("Hello, world!".to_string()), foo.bar);
        assert_eq!(Some(1984), foo.baz);

        Ok(())
    }

    #[test]
    fn builder_test() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Builder)]
        struct Foo {
            bar: String,
            baz: i16,
            qux: bool,
        }

        let foo = Foo::builder()
            .bar("Hello, world!")
            .baz(1984_i16)
            .qux(true)
            .build()?;

        assert_eq!("Hello, world!", foo.bar);
        assert_eq!(1984_i16, foo.baz);
        assert_eq!(true, foo.qux);

        Ok(())
    }
}
