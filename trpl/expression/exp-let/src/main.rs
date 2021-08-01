fn main() {

    {
        fn x(x: i8) -> Result<i8, std::io::Error> {
            if x > 0 {
                Ok(x)
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::AddrInUse, ""))
            }
        }
        let a = x(0);
        println!("{:?}", a);
    }
    
    {
        // use std::io;
        // fn show_files() -> io::Result<()> {
        //     let mut v = vec![];

        // }


    }
    


}
