

#[cfg(test)]
mod test {
    use crabstructor::Constructor;
    #[test]
    fn lib_test() {

        #[derive(Constructor, Eq, PartialEq, Debug)]
        #[constructor(
            field1 = r#"String::from("test")"#
        )]
        struct Example {
            field1: String,
            field2: i32
        }

        assert_eq!(Example::new(2), Example {field1: "test".to_string(), field2: 2})
    }

    #[test]
    fn lib_test_2() {

        #[derive(Constructor, Eq, PartialEq, Debug)]
        #[constructor(
            field1 = r#"String::from("test")"#
        )]
        struct Example {
            field1: String,
            field2: i32
        }

        #[derive(PartialEq,Eq,Debug)]
        struct Foo (bool);
        impl Default for Foo {
            fn default() -> Self {
                Self(true)
            }
        }

        #[derive(Constructor, Eq, PartialEq, Debug)]
        #[constructor(
            field1 = r#"String::from("test")"#
        )]
        #[constructor(
            field2,field4,field5
        )]
        #[constructor(
            field3 = r#"Example::new(-13)"#
        )]
        struct Example2 {
            field1: String,
            field2: i32,
            field3: Example,
            field4: i8,
            field5: Foo,
        }

        let expected_nested = Example {field1: "test".to_string(), field2: -13};
        assert_eq!(Example2::new(), Example2 {field1: "test".to_string(), field2: 0, field3: expected_nested, field4: 0, field5: Foo(true)})

        /// TODO rust-analyzer thinks Example2::new() constructor still takes the field2,field4 and field5 arguments which are using the same
        /// Type::default() way instead of field_name = ....
        /// which it does recognize are not part of the constructor
        /// but the test above still works
        /// ?????
    }
}

fn main() {}