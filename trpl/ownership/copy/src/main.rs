fn main() {
    
    #[derive(Copy, Clone)]
    struct Label { number: u32 }
    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }
    let l = Label {number: 0};
    print(l);
    println!("My label number is: {}", l.number);

    // #[derive(Copy, Clone)]
    // struct StringLabel { name: String }
    // fn print_s(l: StringLabel) {
    //     println!("STAMP: {}", l.name);
    // }
    // let sl = StringLabel {name: "string name".to_string()};
    // print_s(sl);
    // println!("My StringLabel name is: {}", l.name);
}
