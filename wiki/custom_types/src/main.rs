// 隐藏对未使用代码的警告
#![allow(dead_code)]

fn main() {
    // struct
    {
        #[derive(Debug)]
        struct Person<'a> {
            name: &'a str,
            age: u8,
        }

        // 单元结构体
        #[derive(Debug)]
        struct Nil;

        // 元组结构体
        struct Pair(i32, f32);

        // 带有两个字段的结构体
        #[derive(Debug)]
        struct Point {
            x: f32,
            y: f32,
        }

        // 结构体作为另一个结构体的字段
        #[derive(Debug)]
        struct Rectangle {
            p1: Point,
            p2: Point,
        }

        let name = "Peter";
        let age = 27;
        let peter = Person { name, age };
        println!("{:?}", peter);

        let point: Point = Point { x: 0.3, y: 0.7 };
        println!("point coordinates: ({}, {})", point.x, point.y);

        let Point { x: my_x, y: my_y } = point;
        println!("let x: {}, y: {}", my_x, my_y);

        let rectangle = Rectangle {
            p1: Point { x: my_x, y: my_y },
            p2: point,
        };
        println!("rectangle is {:?}", rectangle);

        let nil = Nil;
        println!("nil is {:?}", nil);

        let pair = Pair(1, 0.1);
        println!("pair contains {:?} and {:?}", pair.0, pair.1);

        let Pair(integer, decimal) = pair;
        println!("pair contains {:?} and {:?}", integer, decimal);

        #[derive(Debug)]
        struct Square {
            width: f32,
            height: f32,
        }
        fn rect_area(s: &Square) -> f32 {
            s.width * s.height
        }
        let s = Square {
            width: 10f32,
            height: 100f32,
        };
        println!("square: {:?}, area: {}", s, rect_area(&s));

        #[derive(Debug)]
        struct NewRectangle {
            w: f32,
            h: f32,
            p: Point,
        }
        fn square(p: Point, n: f32) -> NewRectangle {
            NewRectangle { w: n, h: n, p: p }
        }
        println!(
            "new rectangle: {:?}",
            square(Point { x: 1f32, y: 1f32 }, 10f32)
        );
    }

    println!("----------------------------------------------------------------");

    // enum
    {
        enum WebEvent {
            PageLoad,
            PageUnload,
            KeyPress(char),
            Paste(String),
            Click { x: i64, y: i64 },
        }

        fn inspect(event: WebEvent) {
            match event {
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                WebEvent::KeyPress(c) => println!("pressed '{}'", c),
                WebEvent::Paste(s) => println!("pasted \"{}\"", s),
                WebEvent::Click { x, y } => println!("clicked at x={}, y={}.", x, y),
            }
        }

        let pressed = WebEvent::KeyPress('x');
        let pasted = WebEvent::Paste("my text".to_owned());
        let click = WebEvent::Click { x: 20, y: 30 };
        let load = WebEvent::PageLoad;
        let unload = WebEvent::PageUnload;

        inspect(pressed);
        inspect(pasted);
        inspect(click);
        inspect(load);
        inspect(unload);

        #[allow(dead_code)]
        #[derive(Debug)]
        enum VeryVerboseEnumOfThingsToDoWithNumbers {
            Add,
            Sub,
        }
        impl VeryVerboseEnumOfThingsToDoWithNumbers {
            fn run(&self, x: i32, y: i32) -> i32 {
                match self {
                    Self::Add => x + y,
                    Self::Sub => x - y,
                }
            }
        }

        type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;
        let op_add = Operations::Add;
        println!("Operations Add: {:?}", op_add);
        println!(
            "Operations Subtract: {:?}",
            (Operations::Sub).run(100i32, 30i32)
        );
    }

    println!("--------------------------------------------------------");

    // enum -> use
    {
        enum Status {
            Rich,
            Poor,
        }

        enum Work {
            Civilian,
            Soldier,
        }

        use Status::{Poor, Rich};
        use Work::*;

        let status = Poor;
        let work = Civilian;

        match status {
            Rich => println!("The rich have lots of money!"),
            Poor => println!("The poor have no money..."),
        }
        match work {
            Civilian => println!("Civilians work!"),
            Soldier => println!("Soldier fight!"),
        }
    }

    println!("-------------------------------------------");

    // enum -> c style
    {
        enum Number {
            A,
            B,
            C,
        }

        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }

        println!("zero is {}", Number::A as i32);
        println!("one is {}", Number::B as i32);

        println!("rose are #{:06x}", Color::Red as i32);
        println!("violets are #{:06x}", Color::Blue as i32);
    }

    println!("-------------------------------------------");

    // enum -> list
    {
        use List::*;

        enum List {
            Cons(u32, Box<List>),
            Nil,
        }

        impl List {
            // 创建空的List
            fn new() -> List {
                Nil
            }

            // 头部添加新元素
            fn prepend(self, elem: u32) -> List {
                Cons(elem, Box::new(self))
            }

            // List长度
            fn len(&self) -> u32 {
                match self {
                    Cons(_, tail) => 1 + tail.len(),
                    Nil => 0,
                }
            }

            // 列表字符串
            fn stringify(&self) -> String {
                match self {
                    Cons(head, tail) => {
                        format!("{} {}", head, tail.stringify())
                    }
                    Nil => {
                        format!("Nil")
                    }
                }
            }
        }

        let mut list = List::new();
        list = list.prepend(0u32);
        list = list.prepend(1u32);
        list = list.prepend(2u32);

        println!("len: {}, string: {}", list.len(), list.stringify());
    }

    println!("---------------------------\n");

    // const
    {
        static mut LANGUAGE: &'static str = "Rust";
        const THRESHOLD: i32 = 10;

        fn is_big(n: i32) -> bool {
            n > THRESHOLD
        }

        let n = 16;
        unsafe {
            println!("This is {}", LANGUAGE);
        }
        println!("the threshold is {}", THRESHOLD);
        println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

        unsafe {
            LANGUAGE = "C";
            println!("new language is {}", LANGUAGE);
        }
    }
}
