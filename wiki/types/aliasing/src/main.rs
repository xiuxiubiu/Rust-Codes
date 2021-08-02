fn main() {
    type NanoSecond = u64;
    type Inch = u64;

    #[allow(non_camel_case_types)]
    // type u64_t = u64;

    // `NanoSecond` = `Inch` = `u64_t` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;

    // 注意类型别名*并不能*提供额外的类型安全，因为别名*并不是*新的类型。
    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
