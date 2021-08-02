fn main() {
    // from
    {
        let my_str = "hello";
        let my_string = String::from(my_str);
        println!("my_str: {}, my_string: {}", my_str, my_string);

        #[derive(Debug)]
        struct Number {
            value: i32,
        }
        impl std::convert::From<i32> for Number {
            fn from(item: i32) -> Self {
                Number { value: item }
            }
        }

        #[derive(Debug)]
        struct NewNumber {
            number: i32,
        }
        impl From<i32> for NewNumber {
            fn from(value: i32) -> Self {
                NewNumber { number: value }
            }
        }

        #[derive(Debug)]
        struct NotImplFrom {
            number: i32,
        }

        let num = Number::from(30);

        println!("My number is: {:?}", num);

        let number: Number = 300i32.into();
        println!("The number is: {:?}", number);

        let new_numnber: NewNumber = 1000i32.into();
        println!("New number is: {:?}", new_numnber);

        // let notImpl: NotImplFrom = 1000i32.into();
    }
}
