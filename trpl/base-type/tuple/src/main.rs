fn main() {
    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);

    let tmp = text.split_at(21);

    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    println!("head: {}, tail: {}", tmp.0, tmp.1);

    let t = ("tuple",);
    println!("{}", t.0);

    let tuple = ((1, "abc"), 2, '*');
    println!("{}", tuple.0 .0);
    println!("{}", tuple.0 .1);
}
