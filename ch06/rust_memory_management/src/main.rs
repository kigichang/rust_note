fn main() {
    // {
    //     let a = 10;
    //     let b = a; // Copy

    //     assert_eq!(a, 10);
    //     assert_eq!(b, 10);

    //     let a = format!("{:p}", &a); // 取得記憶體位置
    //     let b = format!("{:p}", &b); // 取得記憶體位置
    //     println!("{:?}", a);
    //     println!("{:?}", b);
    //     assert_ne!(a, b); // 不同記憶體位置

    //     let s1 = "hello".to_string();
    //     let s2 = s1; // Move
    //     assert_eq!(s2, "hello");
    //     //println!("{:?}", s1); // Error
    // }

    // {
    //     let s1 = "hello".to_string();
    //     let s2 = s1;
    //     println!("{:?}", s1); // Error
    // }

    // {
    //     fn show(s: String) {
    //         println!("{}", s);
    //     }

    //     let s = "hello".to_string();
    //     show(s);
    //     println!("{:?}", s); // Error
    // }

    // {
    //     let hello = "hello".to_string();
    //     let my_print = || {
    //         println!("{}", hello);
    //         hello
    //     };

    //     my_print();
    //     println!("{:?}", hello);
    // }

    // {
    //     let a = Some(10);
    //     if let Some(x) = a {
    //         // Copy
    //         println!("is some of {}", x);
    //     } else {
    //         println!("is none");
    //     }
    //     println!("{:?}", a);

    //     let a = Some("abc".to_string());

    //     if let Some(ref x) = a {
    //         // 解構取出 Inner Reference
    //         assert_eq!(x, "abc");
    //     } else {
    //         assert!(false);
    //     }
    //     println!("{:?}", a); // 沒事

    //     if let Some(x) = a {
    //         // 解構取值
    //         assert_eq!(x, "abc");
    //     } else {
    //         assert!(false);
    //     }
    //     println!("{:?}", a); // Error
    // }

    // {
    //     let v1 = vec![1, 2, 3];

    //     for x in &v1 {
    //         println!("{}", x);
    //     }

    //     println!("{:?}", v1); // Error

    //     for x in v1 {
    //         println!("{}", x);
    //     }
    //     println!("{:?}", v1); // Error
    // }

    // let g = "global".to_string();
    // {
    //     fn add(mut s: String) -> String {
    //         s.push_str(" world");
    //         s
    //     }

    //     let s = "hello".to_string();
    //     let s2 = add(s);
    //     assert_eq!(s2, "hello world");

    //     fn add2(s: &mut String) {
    //         s.push_str(" world");
    //     }

    //     let mut s = "hello".to_string();
    //     add2(&mut s);
    //     assert_eq!(s, "hello world");

    //     fn f2(mut x: &String) {
    //         let mut y = x;
    //         println!("{}", y);
    //         let c = String::from("c");
    //         y = &c;
    //         println!("{}", y);
    //     }
    //     let x = &s;
    //     f2(x);
    //     println!("{:?}", s);
    // }

    // {
    //     // 1. 賦值至另一個變數
    //     let s1 = "hello".to_string();
    //     let s2 = s1;
    //     //  println!("{:?}", s1); // Error

    //     // 2. 傳遞至函式
    //     fn show(s: String) {
    //         println!("{}", s);
    //     }

    //     let s = "hello".to_string();
    //     show(s);
    //     //println!("{:?}", s); // Error

    //     // 3. 被移入 Closure
    //     let mut hello = "hello".to_string();
    //     let mut append = move |s: &str| {
    //         hello.push_str(s);
    //     };
    //     append(" world");
    //     // println!("{}", hello); // Error
    // }

    // {
    //     let a = 10;
    //     let aa = &a;
    //     assert_eq!(*aa, 10);
    // }

    // {
    //     let mut a = 10;
    //     let aa = &mut a;

    //     *aa = 20;
    //     assert_eq!(*aa, 20);
    //     println!("{}", a);
    //     a = 30;
    //     assert_eq!(*aa, 30);
    // }

    // {
    //     // 可以有多個 immutable reference
    //     let a = 10;
    //     let a1 = &a;
    //     let a2 = &a;

    //     assert_eq!(*a1, a);
    //     assert_eq!(*a2, a);
    // }

    // {
    //     let mut a = 10;
    //     let aa = &mut a;
    //     *aa = *aa * 2;
    //     assert_eq!(a, 20);
    //     //println!("{}", *aa);
    //     println!("{}", a);
    // }

    // {
    //     let mut a = 10;
    //     {
    //         let aa = &mut a;
    //         *aa = *aa * 2;
    //     }
    //     assert_eq!(a, 20);
    // }

    // {
    //     let mut a = 10;
    //     let a1 = &a;
    //     let a2 = &mut a; // Error

    //     *a2 = *a1 * 2;
    //     println!("{}", a1);
    // }

    // {
    //     let mut a = 10;
    //     let a2 = &mut a;
    //     *a2 = *a2 * 2;
    //     let a1 = &a; // Error
    //     let a3 = &mut a; // Error

    //     println!("{}", a2);
    // }

    // {
    //     let s;
    //     {
    //         let s1 = "hello".to_string();
    //         s = &s1; //`s1` does not live long enough
    //     }
    //     println!("{}", s);
    // }

    // {
    //     fn longest(x: &str, y: &str) -> &str {
    //         if x.len() > y.len() {
    //             x
    //         } else {
    //             y
    //         }
    //     }

    //     let s1 = "hello".to_string();
    //     let s2 = "world".to_string();
    //     let result = longest(&s1, &s2);
    //     println!("{}", result);
    // }

    // {
    //     fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //         if x.len() > y.len() {
    //             x
    //         } else {
    //             y
    //         }
    //     }

    //     let s1 = "abc".to_string();

    //     {
    //         let s2 = "world".to_string();

    //         {
    //             let result = longest(&s1, &s2);
    //             println!("{}", result);
    //         }
    //     }
    // }

    {
        fn longest<'c, 'a: 'c, 'b: 'c>(x: &'a str, y: &'b str) -> &'c str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let s1 = "abc".to_string();
        {
            let s2 = "world".to_string();
            {
                let result = longest(&s1, &s2);
                println!("{}", result);
            }
        }
    }

    {
        fn longest<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
        where
            'a: 'c,
            'b: 'c,
        {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let s1 = "abc".to_string();
        {
            let s2 = "world".to_string();
            {
                let result = longest(&s1, &s2);
                println!("{}", result);
            }
        }
    }
}
