fn main() {
    struct Person {
        name: String,
        age: u32,
    }

    // {
    //     let x = Some("hello".to_string());

    //     if let Some(ref s) = x {
    //         println!("{}", s);
    //     } else {
    //         println!("x is None");
    //     }

    //     println!("{:?}", x);

    //     if let Some(s) = &x {
    //         println!("{}", s);
    //     } else {
    //         println!("x is None");
    //     }

    //     println!("{:?}", x);

    //     if let Some(s) = x {
    //         println!("{}", s);
    //     } else {
    //         println!("x is None");
    //     }
    //     println!("{:?}", x); // x is moved
    // }

    // {
    //     let p = ("a".to_string(), "b".to_string());
    //     let (x, y) = p;
    //     println!("{:?}", x);
    //     println!("{:?}", y);
    //     println!("{:?}", p); // p is moved
    // }

    // {
    //     let p = (1_i32, 2_i32);
    //     let (x, y) = p;
    //     println!("{:?}", x);
    //     println!("{:?}", y);
    //     println!("{:?}", p);

    //     let p = ("a".to_string(), "b".to_string());
    //     let (ref x, ref y) = p;
    //     println!("{:?}", x);
    //     println!("{:?}", y);
    //     println!("{:?}", p);
    // }

    // {
    //     let mut x = Some("hello".to_string());
    //     if let Some(ref mut s) = x {
    //         s.push_str(", world");
    //     } else {
    //         println!("x is None");
    //     }
    //     println!("{:?}", x);
    // }

    // {
    //     let mut p = ("a".to_string(), "b".to_string());
    //     let (ref mut x, ref mut y) = p;

    //     x.push_str("c");
    //     y.push_str("d");

    //     println!("{:?}", p);
    // }

    // {
    //     let p = (10, 20);
    //     if let (5, y) = p {
    //         println!("y: {}", y);
    //     } else if let (x @ 1..=30, _) = p {
    //         println!("x: {}", x);
    //     } else {
    //         println!("p: {:?}", p);
    //     }

    //     let x = Some(10);

    //     if let Some(x @ 20) = x {
    //         println!("s: {}", x);
    //     } else {
    //         println!("x: {:?}", x);
    //     }
    // }

    // {
    //     let a = 10_i32;

    //     match a {
    //         1..=9 => println!("1..=9"),
    //         10..=20 => println!("10..=20"),
    //         _ => println!("default"),
    //     }
    // }

    {
        let a = 10_i32;

        let b = match a {
            a @ 1..=9 => a + 10,
            a @ 10..=20 => a * 10,
            a @ _ => a,
        };

        println!("{:?}", b); // 100
    }

    // {
    //     let a = Some(10_i32);

    //     match a {
    //         Some(x) if x > 5 => println!("x: {}", x),
    //         Some(x) if x < 5 => println!("x: {}", x),
    //         Some(x) => println!("x: {}", x),
    //         None => println!("None"),
    //     }

    //     println!("{:?}", a);

    //     let a = Some("abc".to_string());

    //     match a {
    //         Some(x) if x.len() > 5 => println!("x: {}", x),
    //         None => println!("None"),
    //         _ => println!("x: {}", a.unwrap()),
    //     }

    //     println!("{:?}", a); // error
    // }

    // {
    //     let a = Some(10_i32);

    //     let b = match a {
    //         Some(x) if x > 5 => 50,
    //         Some(x) if x < 5 => 10,
    //         Some(x) => x,
    //         None => 0,
    //     };

    //     println!("{:?}", a);
    //     println!("{:?}", b);
    // }

    // {
    //     let a = Some("abc".to_string());

    //     match &a {
    //         Some(x) if x.len() > 5 => println!("x: {}", x),
    //         None => println!("None"),
    //         Some(x) => println!("x: {}", x),
    //     }

    //     println!("{:?}", a);
    // }

    // {
    //     #[derive(Debug)]
    //     enum Either<L, R> {
    //         Left(L),
    //         Right(R),
    //     }

    //     let a: Either<i32, f64> = Either::Left(10_i32);

    //     match a {
    //         Either::Left(x) if x > 5 => println!("x: {}", x),
    //         Either::Left(x) if x < 5 => println!("x: {}", x),
    //         Either::Left(x) => println!("x: {}", x),
    //         Either::Right(x) => println!("x: {}", x),
    //     }

    //     let a: Either<i32, f64> = Either::Right(10.01_f64);
    //     match a {
    //         Either::Left(x) if x > 5 => println!("x: {}", x),
    //         Either::Left(x) if x < 5 => println!("x: {}", x),
    //         Either::Left(x) => println!("x: {}", x),
    //         Either::Right(x) => println!("x: {}", x),
    //     }
    // }

    {
        fn aa(a: &'static str) -> &'static str {
            a
        }

        let a = "aa".to_string();

        let c = aa(&a);
    }
}
