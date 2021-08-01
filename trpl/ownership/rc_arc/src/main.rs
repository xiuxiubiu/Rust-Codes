use std::rc::Rc;

fn main() {
    
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u = t.clone();

    assert!(s.contains("shirataki"));
    assert_eq!(u.find("sh"), Some(0));
    println!("{}", t);

    // s.push_str("ddd");

    

}
