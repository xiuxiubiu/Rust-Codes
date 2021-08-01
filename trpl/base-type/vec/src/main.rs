fn main() {
    
    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    v.push(11);
    v.push(33);
    assert_eq!(v.iter().fold(1, |a, b| a * b), 76230);

    let buffer = new_pixel_buffer(10, 2);
    assert_eq!(buffer.len(), 20);
    
    let mut v1 = Vec::new();
    v1.push("step");
    v1.push("on");
    v1.push("no");
    v1.push("pets");
    assert_eq!(v1, vec!["step", "on", "no", "pets"]);

    let v2: Vec<i32> = (0..5).collect();
    assert_eq!(v2, [0, 1, 2, 3, 4]);

    let mut re = vec!["a man", "a plan", "a canal", "panama"];
    re.reverse();
    assert_eq!(re, vec!["panama", "a canal", "a plan", "a man"]);

    let mut vwc = Vec::with_capacity(2);
    assert_eq!(vwc.len(), 0);
    assert_eq!(vwc.capacity(), 2);
    vwc.push(1);
    assert_eq!(vwc.len(), 1);
    vwc.push(2);
    assert_eq!(vwc.capacity(), 2);
    vwc.push(3);
    assert_eq!(vwc.len(), 3);
    assert_eq!(vwc.capacity(), 4);

    let mut v3 = vec![10, 20, 30, 40, 50];
    v3.insert(3, 35);
    assert_eq!(v3.get(3), Some(&35));
    assert_eq!(v3, [10, 20, 30, 35, 40, 50]);
    v3.remove(3);
    assert_eq!(Some(&40), v3.get(3));
    assert_eq!(v3, [10, 20, 30, 40, 50]);

    let mut v4 = vec!["carmen", "miranda"];
    assert_eq!(v4.pop(), Some("miranda"));
    assert_eq!(v4.pop(), Some("carmen"));
    assert_eq!(v4.pop(), None);

    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!("{}: {}", l, if l.len() % 2 == 0 {"functional"} else {"imperative"});
    }
}

fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
    vec![0; rows * cols]
}