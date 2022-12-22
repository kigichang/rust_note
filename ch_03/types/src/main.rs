fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct S {
    x:f32,
    y:f32,
}

#[derive(Debug)]
struct T (i32, char);

#[derive(Debug)]
struct U;

// -----------------------------------------------------------------------------

#[derive(Debug)]
enum Attend {
    OnTime,
    Late(u32),
}

// -----------------------------------------------------------------------------

fn main() {
    println!("Hello, world!");
    
    let x = {
        let y = 1;
        y + 1
    };

    println!("x = {}", x);

    {
        println!("build_vector() = {:?}", build_vector());
    }

    {
        let t1 = (1i32, 2f64, 3u8);
        println!("t1 = {:?}", t1);
    }

    {
        let s1 = S { x: 1.0, y: 2.0 };
        let s2 = T(1, 'a');
        let s3 = U;

        println!("s1.x = {}, s1.y = {}", s1.x, s1.y);
        println!("s1 = {:?}", s1);
        println!("s2 = {:?}", s2);
        println!("s3 = {:?}", s3);
    }

    {
        let a1 = Box::new(Attend::Late(10));
        let a2 = Attend::OnTime;

        println!("a1 = {:?}", a1);
        println!("a2 = {:?}", a2);
    }

    {
        println!("{}", "ラーメン: ramen".to_string());
    }

    {
        let s = "そば: soba";
        let _s1 = &s[0..3];

        println!("{}", s);
        println!("{}", &s[0..3]);
        println!("{}", &s[3..6]);
        println!("{}", &s[6..7]);
    }

    {
        let a1 = [1, 2, 3, 4];
        println!("a1 = {:?}", a1);
    }

    {
        let a = 116i8;
        let b = 0xcafeu32;
        let c = 0b0010_1010;
        let d = 0o106;

        println!("a = {}", a);
        println!("b = {}", b);
        println!("c = {}", c);
        println!("d = {}", d);
    }

    {
        let a = b'\'';
        let b = b'\\';
        let c = b'\n';
        let d = b'\r';
        let e = b'\t';

        println!("a = {}", a);
        println!("b = {}", b);
        println!("c = {}", c);
        println!("d = {}", d);
        println!("e = {}", e);
    }

    {
        assert_eq!(10_i8 as u16, 10_u16);       // in range
        assert_eq!(2525_u16 as i16, 2525_i16);  // in range
        
        assert_eq!(-1_i16 as i32, -1_i32);          // sign-extended
        assert_eq!(65535_u16 as i32, 65535_i32);    // zero-extended
    }

    {
        assert_eq!(1_000_i16 as u8, 232_u8);
        assert_eq!(65535_u32 as i16, -1_i16);
        assert_eq!(-1_i8 as u8, 255_u8);
        assert_eq!(255_u8 as i8, -1_i8);
    }

    {
        assert_eq!(2_u16.pow(4), 16);
        assert_eq!((-4_i32).abs(), 4);
        assert_eq!(0b101101_u8.count_ones(), 4);
        assert_eq!(0b101101_u8.count_zeros(), 8 - 0b101101_u8.count_ones());
    }

    {
        //println!("{}", -4.abs());
        println!("{}", -4_i32.abs());
    }

    {
        //let mut i: i32 = 1;
        //
        //loop {
        //    //i *= 10;
        //    i = i.checked_mul(10).expect("multiplication overflow");
        //}

        assert_eq!(10_u8.checked_add(20), Some(30));
        assert_eq!(100_u8.checked_add(200), None);

        assert_eq!((-128_i8).checked_div(-1), None);
    }

    {
        assert_eq!(100_u16.wrapping_mul(200), 20000);
        assert_eq!(500_u16.wrapping_mul(500), 53392);
        assert_eq!(500_i16.wrapping_mul(500), -12144);

        assert_eq!(5_i16.wrapping_shl(17), 10)

    }

    {
        assert_eq!(32760_i16.saturating_add(10), 32767);
        assert_eq!(32760_i16.saturating_add(100), 32767);
        assert_eq!((-32760_i16).saturating_sub(10), -32768);
    }

    {
        assert_eq!(255_u8.overflowing_sub(2), (253, false));
        assert_eq!(255_u8.overflowing_add(2), (1, true));

        assert_eq!(5_u16.overflowing_shl(17), (10, true));
    }

    {
        assert_eq!(100_i8.checked_add(27), Some(127));
        assert_eq!(10_u8.checked_sub(11), None);
        assert_eq!(128_u8.saturating_mul(3), 255);
        assert_eq!(64_u16.wrapping_div(8), 8);
        assert_eq!((-32768_i16).wrapping_rem(-1), 0);
        assert_eq!((-128_i8).checked_neg(), None);
        assert_eq!((-32768_i16).wrapping_abs(), -32768);
        assert_eq!(3_u8.checked_pow(4), Some(81));
        assert_eq!(10_u32.wrapping_shl(34), 40);
        assert_eq!(40_u64.wrapping_shr(66), 10);
    }

    {
        assert!((-1. / f32::INFINITY).is_sign_negative());
        assert_eq!(-f32::MIN, f32::MAX);

        assert_eq!(5_f32.sqrt() * 5_f32.sqrt(), 5_f32);
        assert_eq!((-1.01_f64).floor(), -2.0);

        println!("{}", (2.0_f64).sqrt());
        println!("{}", f64::sqrt(2.0));
    }

    {
        assert_eq!(false as i32, 0);
        assert_eq!(true as i32, 1);
    }

    {
        assert_eq!('*' as i32, 42);
        let c = '\u{CA0}';
        println!("{}", c);
        assert_eq!(c as u16, 0xca0);
        assert_eq!(c as i8, -0x60);
        assert_eq!(c.len_utf8(), 3);
        println!("{}", c.len_utf16());

        assert_eq!('*'.is_alphabetic(), false);
        assert_eq!('β'.is_alphabetic(), true);
        assert_eq!('8'.to_digit(10), Some(8));
        assert_eq!('８'.to_digit(10), None);
        
       assert_eq!(std::char::from_digit(8, 10), Some('8'));
    }

    {
        let text = "I see the eigenvalue in thine eye";
        let (head, tail) = text.split_at(21);
        assert_eq!(head, "I see the eigenvalue ");
        assert_eq!(tail, "in thine eye");

        let tmp = text.split_at(21);
        assert_eq!(tmp.0, "I see the eigenvalue ");
        assert_eq!(tmp.1, "in thine eye");
    }

    {
        let lazy_caterer: [u32;6] = [1, 2, 4, 7, 11, 16];
        let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
        assert_eq!(lazy_caterer[3], 7);
        assert_eq!(taxonomy.len(), 3);
    }

    {
        let mut sieve = [true; 10_000];

        for i in 2..100 {
            if sieve[i] {
                let mut j = i * i;
                while j < 10_000 {
                    sieve[j] = false;
                    j += i;
                }
            }
        }

        assert!(sieve[211]);
        assert!(!sieve[9876]);
    }

    {
        let mut chaos = [3, 5, 4, 1, 2];
        chaos.sort();
        assert_eq!(chaos, [1, 2, 3, 4, 5]);
    }

    {
        let mut primes = vec![2, 3, 5, 7];
        assert_eq!(primes.iter().product::<i32>(), 210);

        primes.push(11);
        primes.push(13);
        assert_eq!(primes.iter().product::<i32>(), 30030);
    }

    {
        fn new_pixel_buffer(rows: usize, cols: usize) -> Vec<u8> {
            vec![0; rows * cols]
        }

        println!("{:?}", new_pixel_buffer(2,3));
    }

    {
        let mut pal = Vec::new();
        pal.push("step");
        pal.push("on");
        pal.push("no");
        pal.push("pets");
        assert_eq!(pal, vec!["step", "on", "no", "pets"]);
    }

    {
        let v: Vec<i32> = (0..5).collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }

    {
        let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
        palindrome.reverse();
        assert_eq!(palindrome, vec!["panama", "a canal", "a plan", "a man"]);
    }

    {
        let mut v = Vec::with_capacity(2);
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 2);
        v.push(1);
        v.push(2);
        assert_eq!(v.len(), 2);
        assert_eq!(v.capacity(), 2);

        v.push(3);
        assert_eq!(v.len(), 3);
        println!("capacity is now {}", v.capacity());
    }

    {
        let mut v = vec![10, 20, 30, 40, 50];
        println!("len: {}, cap: {}, val: {:?}", v.len(), v.capacity(), v);
        v.insert(3, 35);
        println!("len: {}, cap: {}, val: {:?}", v.len(), v.capacity(), v);
        assert_eq!(v, vec![10, 20, 30, 35, 40, 50]);

        v.remove(1);
        println!("len: {}, cap: {}, val: {:?}", v.len(), v.capacity(), v);
        assert_eq!(v, vec![10, 30, 35, 40, 50]);
    }

    {
        let mut v = vec!["Snow Puff", "Glass Gem"];
        assert_eq!(v.pop(), Some("Glass Gem"));
        assert_eq!(v.pop(), Some("Snow Puff"));
        assert_eq!(v.pop(), None);
    }

    {
        let languages = vec!["Lisp", "Scheme", "C", "C++", "Fortran"];

        for l in languages {
            println!("{}: {}", l, if l.len() %2 == 0 { "functional" } else { "imperative" });
        }
    }

    {
        let v = vec![0.0_f64, 0.707, 1.0, 0.707];
        let a = [0.0_f64, -0.707, -1.0, -0.707];

        let sv: &[f64] = &v; // explicit
        let sa: &[f64] = &a; // explicit

        fn print(n: &[f64]) {
            for elt in n {
                println!("{}", elt);
            }
        }

        print(sv);
        print(sa);

        print(&v); // implicit &Vec<f64> -> &[f64]
        print(&a); // implicit &[f64; 4] -> &[f64]

        print(&v[0..2]);
        print(&a[2..]);
        print(&sv[1..3]);
    }

    {
        let speech = "\"Ouch!\" said the well.\n";
        println!("{}", speech);

        let str1 = "In the room the women come and go, 
        Singing of Mount Abora";
        println!("{}", str1);

        let str2 = "It was a bright, cold day in April, and \
        there were four of us -\
        more or less.";

        println!("{}", str2);

        let _default_win_install_path = r"C:\Program Files\Gorillas";
        let _pattern = regex::Regex::new(r"\d+(\.\d+)*");

        println!(r###"
            This raw string started with 'r###"'.
            Therefore it does not end until we reach a quote mark ('"')
            followed immediately by three pound signs ('###'):
            "###);
    }

    {
        let method = b"GET";
        assert_eq!(method, &[b'G', b'E', b'T']);
    }

    {
        let noodles = "noodles".to_string();
        let oodles = &noodles[1..];
        println!("{}", oodles);

        let poodles = "ಠ_ಠ";

        assert_eq!(poodles.len(), 7);
        assert_eq!(poodles.chars().count(), 3);
    }

    {
        let error_message = "too many pets".to_string();
        println!("Error: {}", error_message);
        assert_eq!(format!("{}°{:02}‘{:02}“N", 24, 5, 23), "24°05‘23“N");
    }

    {
        let bits = vec!["veni", "vidi", "vici"];
        assert_eq!(bits.concat(), "venividivici");
        assert_eq!(bits.join(", "), "veni, vidi, vici");
    }

    {
        assert_eq!("ONE".to_lowercase(), "one");
        assert!("peanut".contains("nut"));
        assert_eq!("      clean\n".trim(), "clean");

        for word in "veni, vidi, vici".split(", ") {
            assert!(word.starts_with("v"));
        }
    }

    {
        type Bytes = Vec<u8>;
        fn decode(data: &Bytes) {

        }
    }

}
