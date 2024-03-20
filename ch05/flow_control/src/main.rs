fn main() {
    // {
    //     let a;
    //     let b = -1;

    //     if b <= 0 {
    //         a = "".to_string();
    //     } else if b > 0 && b < 10 {
    //         a = format!("0{}", b);
    //     } else {
    //         a = format!("{}", b);
    //     }
    //     assert_eq!(a, "");

    //     // a = b > 0 ? b : 0;
    //     let a = if b > 0 { b } else { 0 };
    //     assert_eq!(a, 0);
    // }

    // {
    //     let pt = Some((10, 20));

    //     if let Some((x, y)) = pt {
    //         println!("x: {}, y: {}", x, y);
    //     } else {
    //         println!("no point exists");
    //     }
    // }

    // {
    //     let a = Some(10);
    //     if let Some(x) = a {
    //         println!("is some of {}", x);
    //     } else {
    //         println!("is none");
    //     }
    //     println!("{:?}", a);

    //     let a = Some("abc".to_string());

    //     if let Some(ref x) = a {
    //         assert_eq!(x, "abc");
    //     } else {
    //         assert!(false);
    //     }
    //     println!("{:?}", a);

    //     if let Some(x) = a {
    //         assert_eq!(x, "abc");
    //     } else {
    //         assert!(false);
    //     }
    //     //println!("{:?}", a); // ^ value borrowed here after partial move
    // }

    // {
    //     let mut counter = 0;

    //     loop {
    //         counter += 1;
    //         if counter <= 5 {
    //             continue;
    //         }
    //         println!("{:?}", counter);
    //         if counter == 10 {
    //             break;
    //         }
    //     }
    //     println!("{:?}", counter);
    // }

    // {
    //     let mut counter = 0;
    //     let a = loop {
    //         counter += 1;
    //         if counter == 10 {
    //             break counter / 2;
    //         }
    //     };
    //     println!("{}", a);
    // }

    // {
    //     let mut count = 0;
    //     'counting_up: loop {
    //         println!("count = {count}");
    //         let mut remaining = 10;

    //         loop {
    //             println!("remaining = {remaining}");
    //             if remaining == 9 {
    //                 break;
    //             }
    //             if count == 2 {
    //                 break 'counting_up;
    //             }
    //             remaining -= 1;
    //         }

    //         count += 1;
    //     }
    //     println!("End count = {count}");
    // }
    // {
    //     println!("{}-{}-{}", file!(), line!(), column!());
    // }
    //

    // {
    //     let a = ["a".to_string(), "b".to_string(), "c".to_string()];

    //     let mut idx = 0;
    //     while idx < a.len() {
    //         println!("{}", a[idx]);
    //         let x = a[idx];
    //         idx += 1;
    //     }
    // }

    // {
    //     let mut idx = 0;
    //     let v = vec![1, 2, 3, 4, 5];
    //     while idx < v.len() {
    //         idx += 1;
    //         if idx % 2 == 0 {
    //             continue;
    //         }
    //         println!("{}", v[idx]);
    //         if idx == 3 {
    //             break;
    //         }
    //     }
    // }

    // {
    //     let mut idx = 0;
    //     let v = vec![1, 2, 3, 4, 5];
    //     while idx < v.len() {
    //         if idx == 3 {
    //             break v[idx] * 2;
    //         }
    //     }
    // }

    // {
    //     for i in 0..10 {
    //         println!("{}", i);
    //     }

    //     for i in 0..=10 {
    //         println!("{}", i);
    //     }
    // }

    // {
    //     let v = vec!["a", "b", "c", "d", "e"];
    //     for val in &v {
    //         println!("{}", val);
    //     }
    //     println!("{:?}", v);
    // }

    // {
    //     let v = vec!["a", "b", "c", "d", "e"];
    //     for (idx, val) in v.iter().enumerate() {
    //         println!("idx: {}, val: {}", idx, val);
    //     }
    // }

    // {
    //     fn my_println(x: i32) {
    //         println!("{}", x);
    //     }

    //     fn add(a: i32, b: i32) -> i32 {
    //         a + b
    //     }
    //     my_println(10);
    //     assert_eq!(add(1, 2), 3);
    // }

    {
        fn append(mut s: String) -> String {
            s.push_str(" world");
            s
        }

        let s = "hello".to_string();
        let s = append(s);
        println!("{}", s);

        let mut s = "hello".to_string();
        s = append(s);
        println!("{}", s);
    }
}
