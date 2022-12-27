use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

// -----------------------------------------------------------------------------

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {artist}:");
        for work in works {
            println!(" {work}");
        }
    }
}

// -----------------------------------------------------------------------------

fn show_ref(table: &Table) {
    for (artist, works) in table {
        println!("works by {artist}:");
        for work in works {
            println!(" {work}");
        }
    }
}

// -----------------------------------------------------------------------------

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

// -----------------------------------------------------------------------------

fn factorial(n: usize) -> usize {
    (1..n+1).product()
}

// -----------------------------------------------------------------------------


fn main() {
    {
        let mut table = Table::new();
        table.insert(String::from("Gesualdo"), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
        table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string()]);
        table.insert("Cellini".to_string(), vec!["Perseus with the Head of Medusa".to_string(), "a salt cellar".to_string()]);
        show(table);
    }

    {
        let mut table = Table::new();
        table.insert(String::from("Gesualdo"), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
        table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string()]);
        table.insert("Cellini".to_string(), vec!["Perseus with the Head of Medusa".to_string(), "a salt cellar".to_string()]);
        show(table);
        //assert_eq!(table["Gesualdo"][0], "many madrigals");
    }

    {
        let mut table = Table::new();
        table.insert(String::from("Gesualdo"), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
        table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string()]);
        table.insert("Cellini".to_string(), vec!["Perseus with the Head of Medusa".to_string(), "a salt cellar".to_string()]);
        
        assert_eq!(table["Gesualdo"][0], "many madrigals");
        sort_works(&mut table);
        show_ref(&table);
    }

    {
        let x = 10;
        let r = &x;
        assert_eq!(*r, 10);
    }

    {
        let mut y = 32;
        let m = &mut y;
        *m += 32;
        assert!(*m == 64);
    }

    {
        struct Anime {
            name: &'static str,
            bechdel_pass: bool,
        }

        let aria = Anime {
            name: "Aria: The Animation",
            bechdel_pass: true,
        };

        let anime_ref = &aria;
        assert_eq!((*anime_ref).name, "Aria: The Animation");
        println!("{}", anime_ref.name);
    }

    {
        let x = 10;
        let y = 20;

        let mut r = &x;

        if true {
            r = &y;
        }
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }

        let point = Point { x: 1000, y: 729 };
        let r = &point;
        let rr = &r;
        let rrr = &rr;
        assert_eq!(rrr.y , 729);
    }

    {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);
        assert!(rx == ry);

        assert!(!std::ptr::eq(rx, ry));
    }

    {
        let r = &factorial(6);
        println!("&r = {r}");
        println!("{}", r + 1009);
        println!("{}", r + &1009);
    }

    //{
    //    let r;
    //
    //    {
    //        let x = 1;
    //        r = &x;
    //    }
    //    assert_eq!(*r, 1);
    //}

    {
        static mut STASH: &i32 = &128;

        fn f(p: &'static i32) {
            unsafe {
                STASH = p;
            }
        }

        let x = 100;
        //f(&x);
    }

    {
        fn smallest(v: &[i32]) -> &i32 {
            let mut s = &v[0];

            for r in &v[1..] {
                if r < s {
                    s = r;
                }
            }
            s
        }

        let parabola = [9, 4, 1, 0, 1, 4, 9];
        let s = smallest(&parabola);
        assert_eq!(s, &0);

        //let ss;
        //{
        //    let p = [9, 4, 1, 0, 1, 4, 9];
        //    ss = smallest(&p);
        //}
        //assert!(*ss == 0);
    }
    
}
