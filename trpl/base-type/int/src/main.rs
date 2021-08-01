fn main() {
    
    let big_val = std::i8::MAX;

    let x = big_val.wrapping_add(1);

    println!("{}", x);
}
