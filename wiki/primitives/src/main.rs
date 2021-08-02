fn main() {
    {
        println!("1 + 2 = {}", 1u32 + 2);
        println!("1 - 2 = {}", 1i32 - 2);

        println!("true AND false is {}", true && false);
        println!("true OR false is {}", false || true);
        println!("Not true is {}", !true);

        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0011u32);
        println!("0011 OR 0101 is {:04b}", 0011 | 0101);
        println!("0011 OR 0101 is {:04b}", 0b0011 | 0101);
        println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);

        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80 >> 2);

        println!("One million is written as {}", 1_000_000u32);
    }

    {
        fn reverse(pair: (i32, bool)) -> (bool, i32) {
            let (integer, boolean) = pair;
            (boolean, integer)
        }
        println!("{:?}", reverse((9, true)));

        use std::fmt::*;
        #[derive(Debug)]
        struct Matrix(f32, f32, f32, f32);
        impl Display for Matrix {
            fn fmt(&self, f: &mut Formatter) -> Result {
                writeln!(f, "({} {})", self.0, self.1)?;
                write!(f, "({} {})", self.2, self.3)
            }
        }

        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
        );
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        let tuple_of_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
        println!("tuple of tuple is {:?}", tuple_of_tuple);

        // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
        // println!("too long tuple: {:?}", too_long_tuple);

        let pair = (1, true);
        println!("pair is {:?}", pair);
        println!("the resever pair is {:?}", reverse(pair));

        println!("one element tuple: {:?}", (45u32,));
        println!("just an integer: {:?}", (45u32));

        let tuple = (1, "hello", 4.5, true);
        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);
        println!("{}", matrix);

        fn transpose(m: Matrix) -> Matrix {
            Matrix(m.0, m.2, m.1, m.3)
        }
        let nm = transpose(matrix);
        println!("{}", nm);
    }

    {
        println!("----------------------------------------------------------------");

        use std::mem;

        fn analyze_slice(slice: &[i32]) {
            println!("first element of the slice: {}", slice[0]);
            println!("the slice has {} elements", slice.len());
        }

        let xs: [i32; 5] = [1, 2, 3, 4, 5];
        let ys: [i32; 500] = [0; 500];

        println!("first element of the array: {}", xs[0]);
        println!("second element of the array: {}", xs[1]);

        println!("array size: {}", xs.len());

        println!("array occupies {} bytes", mem::size_of_val(&xs));

        println!("borrow the whole array as a slice");
        analyze_slice(&ys[1..4]);

        // println!("{}", xs[5]);
    }
}
