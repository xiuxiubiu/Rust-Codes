fn main() {
    {
        let r;
        // {
            let x = 1;
            r = &x;
            // assert!(*r == 1);
        // }
        assert_eq!(*r, 1);

    }

    let v = vec![1, 2, 3];
    let r = v[1];
    assert_eq!(r, 2);
    let x= v[1];
    assert_eq!(x, 2);

    

    static mut STASH: &i32 = &128;
    fn f(p: &'static i32) {
        unsafe {
            STASH = p; 
        }
    }

    static WORTH_POINTING_AT: i32 = 1000;
    f(&WORTH_POINTING_AT);
    unsafe {
        assert_eq!(*STASH, WORTH_POINTING_AT);
    }

    f(&111);
    unsafe {
        assert_eq!(*STASH, 111);
    }

    fn g<'a>(_p: &'a i32) {}
    let x= 10;
    g(&x);
    // f(&x);
   
    let s;
    {
        let parabola = [100, 90, 3];
        s = smallest(&parabola);
        assert_eq!(*s, 3);
    }


    struct ST<'a> {
        r: &'a i32
    }
    let st;
    {
        let x = 10;
        st = ST{r: &x};
        assert_eq!(*st.r, 10);
    }

    struct Record<'a> {
        r: &'a [i8]
    }
    fn parse_record<'a>(input: &'a [i8]) -> Record<'a> {
        Record{
            r: input
        }
    }
    let rr = parse_record(&[1, 2, 3]);
    assert_eq!(rr.r, &[1, 2, 3]);

    
    fn modify(v: &mut Vec<i8>) -> &Vec<i8> {
        v[0] = 2;
        v
    }
    let mut v = vec![1, 2, 3];
    modify(&mut v);
    assert_eq!(v, vec![2, 2, 3]);


    struct SS<'a> {
        xs: &'a i32,
        ys: &'a i32,
    }
    let xs = 10;
    let rs;
    {
        let ys = 20;
        {
            let ss = SS { xs: &xs, ys: &ys };
            rs = ss.ys;
            assert_eq!(*rs, 20);
            assert_eq!(*ss.xs, 10);
        }
        assert_eq!(*rs, 20);
    }
    // assert_eq!(*rs, 20);

    struct StringTable {
        elements: Vec<String>
    }
    impl StringTable {
        fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
            for i in 0..self.elements.len() {
                if self.elements[i].starts_with(prefix) {
                    return Some(&self.elements[i]);
                }
            }
            None
        }
    }
    let st = StringTable { elements: vec!["abc".to_string(), "def".to_string()] };
    println!("{:?}", st.find_by_prefix("ab"));


}

fn smallest<'a>(v: &'a [i32]) -> &'a i32 {
    let mut s = &v[0];
    for r in &v[1..] {
        if *r < *s {
            s = r;
        }
    }
    s
}
