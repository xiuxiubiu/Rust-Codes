fn main() {
    print_padovan();

    let point = Box::new((0.625, 0.5));
    let label = format!("{:?}", point);
    assert_eq!(label, "(0.625, 0.5)");

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

struct Person {
    name: String,
    birth: i32,
}

fn print_padovan() {
    let mut padovan = vec![1, 1, 1];
    println!("len:{}, capacity:{}", padovan.len(), padovan.capacity());
    for i in 3..10 {
        let netx = padovan[i - 3] + padovan[i - 2];
        padovan.push(netx);
        println!("len:{}, capacity:{}", padovan.len(), padovan.capacity());
    }
    println!("P(1..10) = {:?}", padovan)
}
