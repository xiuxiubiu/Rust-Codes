fn main() {
    {
        let mut code = 1;
        let one = match code {
            1 => "1",
            2 => "2",
            3 => "3",
            _ => "0",
        };

        assert_eq!(one, "1");

        code = 2;
        let two = if code == 1 { "11" } else { "2" };
        assert_eq!(two, "2");
    }

    {
        let number = Some(7);
        let letter: Option<i8> = None;

        if let Some(i) = number {
            println!("{:?}!", i);
        }

        if let Some(i) = letter {
            println!("{:?}", i);
        } else {
            println!("Didn't match a number. Let's go with a letter!");
        }
    }

    {
        enum Foo {
            Bar,
            _Baz,
            _Qux(u32),
        }

        let a = Foo::Bar;

        // 变量匹配 Foo::Bar
        if let Foo::Bar = a {
            // ^-- 这就是编译时发现的错误。使用 `if let` 来替换它。
            println!("a is foobar");
        }
    }
}
