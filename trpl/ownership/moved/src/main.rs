fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone();
    let u = t;
    assert_eq!(
        s,
        vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]
    );
    assert_eq!(
        u,
        vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]
    );
    // assert_eq!(t, vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]);

    let mut s1 = "Govinda".to_string();
    let t1 = s1;
    s1 = "Siddhartha".to_string();
    assert_eq!(t1, "Govinda");
    assert_eq!(s1, "Siddhartha");

    // struct Person {
    //     name: String,
    //     birth: i32
    // }
    // let mut composers = Vec::new();
    // composers.push(Person { name: "Palestrina".to_string(), birth: 1525});
    // for composer in composers {
    //     println!("{}: {}", composer.name, composer.birth);
    // }

    // fn f(_v: Vec<u8>) {}
    // fn g(_v: Vec<u8>) {}
    // fn h(_v: Vec<u8>) {}
    // let x2 = vec![10, 20, 30, 40];
    // let c = true;
    // if c {
    //     f(x2);
    // } else {
    //     g(x2);
    // }
    // h(x2);

    // fn f3(_v: Vec<u8>) {}
    // let x3 = vec![10, 20, 30, 40];
    // for _i in 0..3 {
    //     f3(x3);
    // }

    // let mut v4 = Vec::new();
    // for i in 1 .. 106 {
    //     v4.push(i.to_string())
    // }
    // let third = v4[2];
    // let fifth = v4[4];

    // let v5 = vec![1, 2, 3];
    // let a = v5[1];
    // let b = v5[1];
    // println!("{}, {}", a, b);

    // let mut v6 = Vec::new();
    // v6.push(1);
    // v6.push(2);
    // let v6_one = v6[1];
    // println!("{}", v6_one);

    // fn f7(_v: &str) {}
    // let n7 = "dddd";
    // let arr7: [&u8; 4] = [&1, &2, &3, &4];
    // f7(n7);
    // let _m7 = arr7[2];
    // println!("{}, {}", n7, arr7[2]);

    // let a8: [[u8;4]; 4] = [[1,2,3,4], [5,6,7,8], [9,10,11,12], [13,14,15,16]];
    // let _m7 = a8[2];
    // println!("{}", a8[2][1])

    let mut v8 = Vec::new();
    for i in 101..106 {
        v8.push(i.to_string());
    }

    let fifth = v8.pop().unwrap();
    assert_eq!(fifth, "105");

    let second = v8.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v8[2], "substitute".to_string());
    assert_eq!(third, "103");

    // for _m in v8 {
    // }
    assert_eq!(v8, vec!["101", "104", "substitute"]);

    let v9 = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    // for mut s in v9 {
    //     s.push('!');
    //     println!("{}", s);
    // }
    assert_eq!(
        v9,
        vec![
            "liberté".to_string(),
            "égalité".to_string(),
            "fraternité".to_string()
        ]
    );

    struct Person {
        name: Option<String>,
        _birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("name".to_string()),
        _birth: 32,
    });
    // let first_name = std::mem::replace(&mut composers[0].name, None);
    let first_name = composers[0].name.take();
    assert_eq!(first_name, Some("name".to_string()));
}
