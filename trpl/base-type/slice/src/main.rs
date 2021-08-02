fn main() {
    let mut v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    assert_eq!(sv.len(), 4);
    assert_eq!(sv[0], 0.0);

    let sa: &[f64] = &a;
    assert_eq!(sa.len(), 4);
    assert_eq!(sa[1], -0.707);
    // assert_eq!(sa, &[0.0, -1.0, -0.707, -0.707][..]);

    print(sa);
    print(sv);

    v.reverse();
    assert_eq!(v, vec![0.707, 1.0, 0.707, 0.0]);

    print(&v[..]);
}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}
