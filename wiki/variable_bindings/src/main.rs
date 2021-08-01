fn main() {
    
    {
        let an_integer = 1u32;
        let a_boolean = false;
        let unit = ();

        let _copied_integer = an_integer;

        println!("An integer: {:?}", an_integer);
        println!("A boolean: {:?}", a_boolean);
        println!("Meet the unit value: {:?}", unit);

        let _no_used = 10i32;
    }

    println!("------------------------------------------------");

    {
        let _immutable_binding = 1u32;
        let mut mutable_binding = 11u32;

        println!("Before mutation: {}", mutable_binding);

        mutable_binding += 1u32;
        println!("After mutation: {}", mutable_binding);

        // _immutable_binding += 1;
    }

    println!("----------------------------------------------------");

    {
        let long_live_binding = 1_u32;

        {
            let short_live_binding = 2_u32;
            println!("inner short: {}", short_live_binding);

            let long_live_binding = 5_u32;
            println!("inner long: {}", long_live_binding);
        }

        println!("outer long: {}", long_live_binding);
        // println!("outer shor: {}", short_live_binding);

        let long_live_binding = "string";
        println!("new long: {}", long_live_binding);
    }

    println!("------------------------------");

    {
        let a_binding;
        {
            let x = 2;
            a_binding = x * x;
            println!("inner a: {}", a_binding);
        }

        println!("outer a: {}", a_binding);

        let another_binding;
        another_binding = 10i32;

        println!("another_binding: {}", another_binding);

    }

    println!("--------------------------------");

    {
        let mut _mutable_integer = 7i32;
        {
            let mut _mutable_integer = _mutable_integer;
            _mutable_integer = 50;
            println!("inner _mutable_integer: {}", _mutable_integer);
        }
        _mutable_integer = 10i32;
        println!("mutable_integer: {}", _mutable_integer);
    }


}
