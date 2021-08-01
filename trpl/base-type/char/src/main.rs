fn main() {
    println!("Hello, world!");

    println!("\'");

    println!("{}", '\x2A');

    println!("{}", '\x11');

    println!("{}", '\u{ABCD}');

    println!("{}", 69_u8 as char);

    println!("{}", match std::char::from_u32(1000) {
        None => {
            std::process::exit(1);
        }
        Some(c) => c
    });

    println!("{}", 'E'.len_utf8());

    println!("{:?}", '*'.to_digit(10_u32));

    println!("{:?}", std::char::from_digit(8, 16))


}
