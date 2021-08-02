fn main() {
    let lazy_caterer: [u32; 6] = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    println!("{}, {}", lazy_caterer[3], taxonomy[2]);

    let mut sieve = [true; 1000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 1000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    println!("{}", sieve[4]);

    let b = [0u8; 1024];

    println!("{}", b[0]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1, 2, 3, 4, 5]);
}
