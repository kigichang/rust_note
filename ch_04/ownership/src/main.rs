fn main() {
    println!("ownership");

    {
        fn print_padovan() {
            let mut padovan = vec![1, 1, 1]; // allocate memory here.
            for i in 3..10 {
                let next = padovan[i - 3] + padovan[i - 2];
                padovan.push(next);
            }

            println!("{:?}", padovan);
            // dropped here.
        }
        print_padovan();
    }

    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        assert_eq!(label, "(0.625, 0.5)");
    }


    {
        struct Person {
            name: String,
            birth: i32,
        }


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

        for composer in &composers {
            println!("{}, born {}", composer.name, composer.birth);
        }
    }
    
    {
        let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
        let t = s;
        //let _u = s;
        //println!("{:?}", s);

        let t1 = t.clone();
        let t2 = t.clone();
        println!("{:?}", t);
        println!("{:?}", t1);
        println!("{:?}", t2);
    }

    {
        let mut s = "Govinda".to_string();
        s = "Siddhartha".to_string(); // value "Govinda" is dropped here.
        println!("{}", s);
    }

    {
        let mut s = "Govinda".to_string();
        let t = s;
        s = "Siddhartha".to_string(); // notthing is dropped here.
        println!("{}", s);
        println!("{}", t);
    }

    {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(i.to_string()); 
        }

        //let third = v[2];
        //let fifth = v[4];

        let fifth = v.pop().unwrap();
        assert_eq!(fifth, "105");

        let second = v.swap_remove(1);
        assert_eq!(second, "102");

        let third = std::mem::replace(&mut v[2], "substitue".to_string());
        assert_eq!(third, "103");
        assert_eq!(v, vec!["101", "104", "substitue"]);
    }

    {
        let mut v = Vec::new();
        for i in 101..106 {
            v.push(Some(i.to_string()));
        }

        let fifth = v[4].take().unwrap();
        assert_eq!(fifth, "105");
        let second = v.swap_remove(1).unwrap();
        assert_eq!(second, "102");
        let third = v[2].take().unwrap();
        assert_eq!(third, "103");
        println!("{:?}", v);

    }

    {
        let v = vec!["liberte".to_string(), "egalite".to_string(), "fraternite".to_string()];

        for mut s in v {
            s.push('!');
            println!("{}", s);
        }
        //println!("{:?}", v);
    }

    {
        struct Person {
            name: Option<String>,
            birth: i32,
        }

        let mut composers = Vec::new();
        composers.push(Person{
            name: Some("Palestrina".to_string()),
            birth: 1525,
        });

        let first_name = std::mem::replace(&mut composers[0].name, None);
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }

    {
        struct Person {
            name: Option<String>,
            birth: i32,
        }

        let mut composers = Vec::new();
        composers.push(Person{
            name: Some("Palestrina".to_string()),
            birth: 1525,
        });

        let first_name = composers[0].name.take();
        assert_eq!(first_name, Some("Palestrina".to_string()));
        assert_eq!(composers[0].name, None);
    }

    {
        #[derive(Clone, Copy, Debug)]
        struct Label {
            number: u32,
        }

        fn print(l: Label) {
            println!("STAMP: {}", l.number);
        }
        
        let l = Label { number: 5 };
        print(l);
        println!("{}", l.number);
    }

    {
        use std::rc::Rc;

        let s = Rc::new("shirataki".to_string());
        let t = s.clone();
        let u = s.clone();

        assert!(s.contains("shira"));
        assert_eq!(t.find("taki"), Some(5));
        println!("{} are quite chewy, almost bouncy, but lack flavor", u);
        //s.push_str("test");
    }
}
