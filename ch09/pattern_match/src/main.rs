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

    {
        let mut p = ("a".to_string(), "b".to_string());
        let (ref mut x, ref mut y) = p;

        x.push_str("c");
        y.push_str("d");

        println!("{:?}", p);
    }

    {
        let p = (10, 20);
        if let (5, y) = p {
            println!("y: {}", y);
        } else if let (x @ 1..=30, _) = p {
            println!("x: {}", x);
        } else {
            println!("p: {:?}", p);
        }

        let x = Some(10);

        if let Some(x @ 20) = x {
            println!("s: {}", x);
        } else {
            println!("x: {:?}", x);
        }
    }
}
