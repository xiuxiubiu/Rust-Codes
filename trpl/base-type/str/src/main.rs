fn main() {
    let speech = "\"Ouch!\" said the well.\n";
    println!("{}", speech);

    println!(
        "In the room the women come and go,
        Singing of Mount Abora"
    );

    println!(
        "It was a bright, cold day in April, and \
        there were four of us—\
        more or less."
    );

    let default_win_install_path = r##"\\测试斜杠""##;
    println!("{}", default_win_install_path);

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();
    assert_eq!(noodles.len(), 7);
    assert_eq!(noodles.capacity(), 7);
    assert_eq!(noodles.chars().count(), 7);

    let oodles = &noodles[1..];
    assert_eq!(oodles, "oodles");
    assert_eq!(oodles.len(), 6);

    let poodles = "確認";
    assert_eq!(poodles.len(), 6);
    assert_eq!(poodles.chars().count(), 2);

    assert_eq!("too many pets".to_string(), "too many pets");

    assert_eq!(format!("{}。{:02}'{:02}N", 24, 5, 23), "24。05'23N");

    let bits = vec!["veni", "vidi", "vici"];
    assert_eq!(bits.concat(), "venividivici");
    assert_eq!(bits.join(","), "veni,vidi,vici");

    let bits_arr = ["veni", "vidi", "vici"];
    assert_eq!(bits_arr.concat(), "venividivici");

    assert!("ONE".to_lowercase() == "one");
    assert!("peanut".contains("nut"));
    assert_eq!("abcde".replace("cd", "dc"), "abdce");
    assert_eq!("  clean\n".trim(), "clean");
}
