fn main() {
    
    let x = 10;
    let r = &x;
    assert!(*r == 10);
    assert_eq!(*r, 10);

    let mut y = 32;
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);

    struct Anime { name: &'static str, _bechdel_pass: bool }
    let aria = Anime{ name: "Aria: The Animation", _bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");
    assert_eq!((*anime_ref).name, "Aria: The Animation");
    
    let mut v = vec![1973, 1968];
    v.sort();
    // (&mut v).sort();
    assert_eq!(v, vec![1968, 1973]);


    let x1 = 10;
    let y1 = 20;
    let mut _r1 = &x1;
    _r1 = &y1;
    assert!(*_r1 == 10 || *_r1 == 20);
    assert_eq!(*_r1, 20);


    struct Point { x: i32, y: i32 }
    let point = Point { x: 1000, y: 729 };
    let p: &Point = &point;
    let pp: &&Point = &p;
    let ppp: &&&Point = &pp;
    assert_eq!(ppp.x, 1000);
    assert_eq!(ppp.y, 729);


    let x2 = 10;
    let y2 = 10;
    let rx2 = &x2;
    let ry2 = &y2;
    let rrx2 = &rx2;
    let rry2 = &ry2;
    let rx2_1 = &x2;
    assert!(rrx2 <= rry2);
    assert!(rrx2 == rry2);
    assert!(rx2 == ry2);
    assert_eq!(rx2, ry2);
    assert!(std::ptr::eq(rx2, rx2_1));

    
    let r = &factorial(6);
    assert_eq!(r + 1009, 1729);
    // println!("{}", r + 100);

}

fn factorial(n: usize) -> usize {
    (1..n+1).fold(1, |a, b| a * b)
}
