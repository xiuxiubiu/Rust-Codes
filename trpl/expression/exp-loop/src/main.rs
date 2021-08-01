fn main() {
    
    {
        let mut x = 0;
        loop {
            if x == 1000 {
                println!("stop");
                break;
            }
            x += 1;
        }
    }

    {
        // for i in 0..20 {
        //     println!("{}", i);
        // }

        // let r = std::ops::Range { start: 0, end: 20 };
        // for i in r {
        //     println!("{}", i)
        // }
        // assert_eq!(r, std::ops::Range { start: 0, end: 20 });

        let mut strings: Vec<String> = 
            vec!["a".to_string(), "b".to_string()];
        for rs in &strings {
            println!("String {:?} is at address {:p}.", *rs, rs);
        }
        assert_eq!(strings, vec!["a".to_string(), "b".to_string()]);

        for rs in &mut strings {
            rs.push('\n');
            println!("String {:?} is at address {:p}.", *rs, rs);
        }

        strings.push("c".to_string());
        'ex:
        for rs in &strings {
            if rs == "b\n" {
                break 'ex;
            }
            println!("String {} is at address {:p}.", *rs, rs);
        }
    }

    {
        let mut optional = Some(0);
        loop {
            match optional {
                Some(i) => {
                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    } else {
                        println!("`i` is `{:?}`. Try again.", i);
                        optional = Some(i + 1);
                    }
                }
                _ => {
                    break;
                }
            }
        }

        optional = Some(0);
        while let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9, quit!");
                optional = None;
            } else {
                println!("`i` is `{:?}`. Try again.", i);
                optional = Some(i + 1);
            }
        }

        
    }
}
