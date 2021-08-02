fn main() {
    {
        let v = vec![1, 2, 3, 4];
        let r = &v;
        assert_eq!(v[0], 1);
        assert_eq!(r[0], 1);
        let aside = v;
        assert_eq!(aside[0], 1);
        // assert_eq!(v[0], 1);
        // assert_eq!(r[0], 1);
    }

    {
        fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
            for elt in slice {
                vec.push(*elt);
            }
        }

        let v: &mut Vec<f64> = &mut vec![1.0];
        extend(v, &[2.0]);
        assert_eq!(v, &vec![1.0, 2.0]);

        let mut y: Vec<f64> = vec![1.0];
        extend(&mut y, &[3.0]);
        assert_eq!(y, vec![1.0, 3.0]);
    }

    {
        // let mut x = 10;
        // let r1 = &x;
        // let r2 = &x;
        // assert!(x == 10 && *r1 == 10 && *r2 == 10);
        // x += 10;
        // assert_eq!(x, 20);
        // let m = &mut x;
        // assert_eq!(*m, 20);

        let mut x = 10;
        let _r = &x;
        let m = &mut x;
        assert!(*m == 10);

        // let mut y = 20;
        // let _m1 = &mut y;
        // let _m2 = &mut y;

        // let mut w = (107, 108);
        // let r = &w;
        // let r0 = &r.0;
        // let m1 = &mut r.1;
    }

    {
        struct File {
            _descriptor: i32,
        }

        fn new_file(d: i32) -> File {
            File { _descriptor: d }
        }

        fn _clone_from(this: &mut File, rhs: &File) {
            this._descriptor = rhs._descriptor;
        }

        let mut _f = new_file(1);
        // clone_from(&mut f, &f)
    }

    {
        let mut x = 42;
        let p = &x;
        assert_eq!(*p, 42);
        x += 1;
        // assert_eq!(*p, 42);
        assert_eq!(x, 43);
        // assert_eq!(*p, 42);
    }
}
