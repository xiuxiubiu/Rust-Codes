fn main() {
    let t = (12, "eggs");

    let b = Box::new(t);

    println!("{}, {}", b.0, b.1);
}
