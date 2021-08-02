fn main() {
    {
        let s = format!("{} days", 32);
        println!("{}", s);

        println!("{0}, this is {1}, {1}, this is {0}", "A", "b");

        println!(
            "{A} this is {B}, {1} this is {0}",
            "aa",
            "bb",
            A = "aa",
            B = "bb"
        );
        println!("{} of {:b} people know binary, the other half don't", 1, 2);

        println!("{number:>width$}", number = 1, width = 6);
        println!("{number:>0dd$}", number = 2, dd = 6);

        println!("My name is {0}, {1} {0}", "Bond", "James");

        #[derive(Debug)]
        struct Structurem(i32, i32);
        let a = Structurem(7, 10);
        println!("This struct `{:?}` won't print ...", a);

        println!("Pi is roughly {:.4}", 3.1415926);

        #[derive(Debug)]
        struct Deep(Structurem);
        println!("The Deep is {:?}", Deep(Structurem(8, 10)));

        println!("{:?} month is a year", 12);
        println!(
            "{1:?} {0:?} is the {actor:?} name",
            "Slater",
            "Christian",
            actor = "actor's"
        );
        println!("Now {:?} will print", Structurem(9, 99));
        println!("Now {:?} will print", Structurem(10, 100).1);
    }

    {
        #[derive(std::fmt::Debug)]
        struct Person<'a> {
            name: &'a str,
            age: u8,
        }

        let name = "Peter";
        let age = 28;
        let peter = Person { name, age };
        println!("Peter is {:#?}", peter);
    }

    {
        use std::fmt::*;

        struct Structue(i32);

        impl Display for Structue {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, "{}", self.0)
            }
        }

        println!("display Structue is {}", Structue(8))
    }

    {
        use std::fmt;

        #[derive(Debug)]
        struct MinMax(i32, i32);
        impl fmt::Display for MinMax {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }

        #[derive(Debug)]
        struct Point2D {
            x: f64,
            y: f64,
        }
        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "x: {}, y: {}", self.x, self.y)
            }
        }

        let minmax = MinMax(0, 14);
        println!("Compare structures:");
        println!("Display: {}", minmax);
        println!("Debug: {:?}", minmax);

        let big_range = MinMax(-300, 300);
        let small_range = MinMax(-3, 3);
        println!(
            "The big range is {big} and the small range is {small}",
            big = big_range,
            small = small_range
        );

        let point = Point2D { x: 3.3, y: 7.2 };
        println!("Compare points:");
        println!("Display: {}", point);
        println!("Debug: {:?}", point);

        impl fmt::Binary for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.x)
            }
        }
        println!("What does Point2D look like in binary: {:b}", point);
    }

    {
        use std::fmt::*;

        #[derive(Debug)]
        struct Complex {
            real: f64,
            imag: f64,
        }
        impl Display for Complex {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, "{} + {}i", self.real, self.imag)
            }
        }

        let complex = Complex {
            real: 3.3,
            imag: 7.2,
        };
        println!("Display: {}", complex);
        println!("Debug: {:?}", complex);
    }

    {
        use std::fmt;

        #[derive(Debug)]
        struct List(Vec<i32>);
        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let vec = &self.0;
                write!(f, "[")?;
                for (c, v) in vec.iter().enumerate() {
                    if c != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}:{}", c, v)?;
                }
                write!(f, "]")
            }
        }

        println!("Display: {}", List(vec![1, 2, 3, 4]));
    }

    {
        use std::fmt::{Display, Formatter, Result};

        struct City {
            name: &'static str,
            lat: f32,
            lon: f32,
        }
        impl Display for City {
            fn fmt(&self, f: &mut Formatter) -> Result {
                let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
                let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
                write!(
                    f,
                    "{}: {:.3}°{} {:.3}°{}",
                    self.name,
                    self.lat.abs(),
                    lat_c,
                    self.lon.abs(),
                    lon_c
                )
            }
        }

        #[derive(Debug)]
        struct Color {
            red: u8,
            green: u8,
            blue: u8,
        }
        impl Display for Color {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(
                    f,
                    "RGB ({red}, {green}, {blue}) 0x{:02x}{:02x}{:02x}",
                    red = self.red,
                    green = self.green,
                    blue = self.blue
                )
            }
        }

        for city in [
            City {
                name: "Dublin",
                lat: 53.347778,
                lon: -6.259722,
            },
            City {
                name: "Oslo",
                lat: 59.95,
                lon: 10.75,
            },
            City {
                name: "Vancouver",
                lat: 49.25,
                lon: -123.1,
            },
        ]
        .iter()
        {
            println!("{}", *city);
        }

        for color in [
            Color {
                red: 128,
                green: 255,
                blue: 90,
            },
            Color {
                red: 0,
                green: 3,
                blue: 254,
            },
            Color {
                red: 0,
                green: 0,
                blue: 0,
            },
        ]
        .iter()
        {
            println!("{}", *color)
        }

        println!("Hello {:5}!", "x");
        println!("Hello {:1$}!", "x", 5);
        println!("Hello {1:0$}!", 5, "x");
        println!("Hello {:width$}!", "x", width = 5)
    }
}
