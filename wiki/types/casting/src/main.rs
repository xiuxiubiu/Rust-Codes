#[allow(overflowing_literals)]

fn main() {
    
    {
        let decimal = 65.4321_f32;

        let integer: u8 = decimal as u8;
        println!("integer: {}", integer);

        let character = integer as char;
        println!("character: {}", character);

        println!("Casting: {} -> {} -> {}", decimal, integer, character);

        // 转无符号 不断 加上或者减去std::T::Max+1
        // 1000 - 256 - 256 - 256 = 232
        // 保留最低的8位 除以2的8次方的余数
        // 256 + -1 = 255
         // 256 + -2 = 254
        println!("1000 as a u16 is: {}", 1000 as u8);
        println!("-1 as a u8 is {}", -1i8 as u8);
        println!("-2 as a u8 is {}", -2i8 as u8);
        println!("1000 mod 256 is: {}", 1000 % 256);

        // 转有符号 先转换对应的无符号类型
        // 比如 200 先专成u8 = 200 = 11001000
        // 然后计算补码表示的值 1000000(-128) + 1001000(72) = -56
        println!("128 as a i16 is: {}", 128 as i16);
        println!("128 as a i8 is : {0}, {0:b}", 128 as i8);
        println!("200 as i8 is: {0}, {0:b}", 200 as i8);
        
        // 1000 as u8 = 1000 % 256 = 232 = 11101000
        // 11101000 as i8 = 1000000(-128) + 1101000(104) = -24
        println!("1000 as i8 is: {0}, {0:b}", 1000 as i8);
    }

}
