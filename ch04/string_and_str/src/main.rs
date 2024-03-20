fn main() {
    {
        let h = [
            b'h', b'e', b'l', b'l', b'o', b',', b'w', b'o', b'r', b'l', b'd',
        ];

        let h1 = &h[1..5];
        let h2 = &h[6..];
        //let hx = h[6..]; // the size for values of type `[u8]` cannot be known at compilation time

        println!("{:-2}, {:?}, {:?}", h.len(), h.as_ptr(), h); // 11, 0x7ffddab3f2cd, [104, 101, 108, 108, 111, 44, 119, 111, 114, 108, 100]
        println!("{:-2}, {:?}, {:?}", h1.len(), h1.as_ptr(), h1); //  4, 0x7ffddab3f2ce, [101, 108, 108, 111]
        println!("{:-2}, {:?}, {:?}", h2.len(), h2.as_ptr(), h2); //  5, 0x7ffddab3f2d3, [119, 111, 114, 108, 100]
    }

    {
        //let h: str;
        let h: &str = "hello,world";
    }

    {
        let hello = String::from("hello,world");
        assert_eq!(hello, "hello,world");

        let hello = "hello,world".to_string();
        assert_eq!(hello, "hello,world");

        let hello = String::from_utf8(vec![b'h', b'e', b'l', b'l', b'o']).unwrap();
        assert_eq!(hello, "hello");

        let sparkle_heart = vec![240, 159, 146, 150];
        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
        assert_eq!(sparkle_heart, "💖");
    }

    {
        let sparkle_heart = vec![240, 159, 146, 150];

        let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);

        assert_eq!("💖", sparkle_heart);

        // some invalid bytes
        let input = b"Hello \xF0\x90\x80World";

        let output = String::from_utf8(input.to_vec());
        assert_ne!(output.is_ok(), true);

        let output_with_replace = b"Hello \xEF\xBF\xBDWorld";
        let output = String::from_utf8_lossy(input);
        println!("from_utf8_lossy: {:?}", output);
        assert_eq!(output.as_bytes(), output_with_replace);

        let output = unsafe { String::from_utf8_unchecked(input.to_vec()) };
        println!("from_utf8_unchecked: {:?}", output);
        assert_eq!(output.as_bytes(), input);
    }

    {
        let long_str = "hello,\
        world!\
        ";
        println!("{}, {}", long_str, long_str.len());

        let long_str = "hello,
        world!
        ";
        println!("{}, {}", long_str, long_str.len());

        let raw_str = r"Escapes don't work here: \x3F \u{211D}";
        println!("{}", raw_str);

        // If you need quotes in a raw string, add a pair of #s
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // If you need "# in your string, just use more #s in the delimiter.
        // You can use up to 65535 #s.
        let longer_delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", longer_delimiter);
    }

    {
        let おはよう = "hello,おはよう".to_string();

        let bytes = おはよう.bytes();
        for byte in bytes {
            print!("{:X} ", byte); // 68 65 6C 6C 6F 2C E3 81 8A E3 81 AF E3 82 88 E3 81 86
        }
        println!();

        let chars = おはよう.chars();
        for char in chars {
            print!("{:?}:{:X} ", char, char as u32); // 'h':68 'e':65 'l':6C 'l':6C 'o':6F ',':2C 'お':304A 'は':306F 'よ':3088 'う':3046
        }
        println!();

        let char_indices = おはよう.char_indices();
        for (idx, char) in char_indices {
            print!("{:?}:{}:{:X} ", idx, char, char as u32); // 0:h:68 1:e:65 2:l:6C 3:l:6C 4:o:6F 5:,:2C 6:お:304A 9:は:306F 12:よ:3088 15:う:3046
        }
    }

    {
        let hello = String::from("hello,world");
        let hello = "hello,world".to_string();
        let empty = String::new();

        println!("{:?}, {}, {}", hello, hello.len(), hello.capacity());
        println!("{:?}, {}, {}", empty, empty.len(), empty.capacity());
        assert!(empty.is_empty());

        let ascii_str = "hello,world".to_string();
        let non_ascii_str = "hello,おはよう".to_string();

        assert!(ascii_str.is_ascii());
        assert_ne!(non_ascii_str.is_ascii(), true);
    }

    {
        let s = "foo".to_string();
        assert_eq!(b"foo", s.as_bytes());
        for byte in s.as_bytes() {
            println!("{:X}", byte);
        }

        let mut s = "foo".to_string();
        unsafe {
            let bytes = s.as_bytes_mut();
            bytes[0] = b'b';
            bytes[1] = b'a';
            bytes[2] = b'r';
        }

        assert_eq!("bar", s);
    }

    {
        let s = "hello world!".to_string();
        assert_eq!(s.get(1..), Some("ello world!"));

        let s = "おはよう".to_string();
        assert!(s.get(1..).is_none());
        assert_eq!(s.get(3..), Some("はよう"));

        let mut s = "hello world!".to_string();

        assert!(s.get_mut(1..).is_some());
        assert!(s.get_mut(..100).is_none()); // 超過範圍

        s.get_mut(1..3).map(|s| s.make_ascii_uppercase()); // 將 s[1..3] 轉為大寫
        assert_eq!(s, "hELlo world!");
    }

    {
        let s = "hello world!!!";
        println!("string is {:?}", s);
        println!("empty? {}", s.is_empty());
        println!("ascii? {}", s.is_ascii());

        let s = "💖".to_string();
        println!("string is {:?}", s);
        println!("empty? {}", s.is_empty());
        println!("ascii? {}", s.is_ascii());

        let s = String::new();
        println!("string is {:?}", s);
        println!("empty? {}", s.is_empty());
        println!("ascii? {}", s.is_ascii());

        let おはよう = "hello,おはよう".to_string();

        println!("string is {:?}", おはよう);
        println!("empty? {}", おはよう.is_empty());
        println!("ascii? {}", おはよう.is_ascii());
        println!("is 0-th char boundary of? {}", おはよう.is_char_boundary(0));
        println!("is 6-th char boundary of? {}", おはよう.is_char_boundary(6));
        println!("is 7-th char boundary of? {}", おはよう.is_char_boundary(7));
        let mut おはよう = "おはよう".to_string();

        println!("{:?}", おはよう.pop());
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s); // foobar

        s.push('!');
        println!("{}", s); // foobar!

        s += "!";
        println!("{}", s); // foobar!!

        s = format!("{}-{}", s, s);
        println!("{}", s); // foobar!!-foobar!!
    }

    {
        let mut s = String::new();
        let mut ptr = s.as_ptr();

        for i in 0..100 {
            s.push_str("foo");
            if ptr != s.as_ptr() {
                println!(
                    "{}-idx realloc: {:?} --> {:?} with new capacity {}",
                    i,
                    ptr,
                    s.as_ptr(),
                    s.capacity()
                );
                ptr = s.as_ptr();
            }
        }
    }

    {
        const MAX: usize = 10_000_000;
        let mut s = String::new();
        let t0 = std::time::Instant::now();
        for _ in 0..MAX {
            s += "foo";
        }
        println!(
            "cost: {:?} ms, len: {}, cap: {}",
            t0.elapsed().as_millis(),
            s.len(),
            s.capacity()
        );

        let mut s = String::with_capacity(3 * MAX);
        let t1 = std::time::Instant::now();
        for _ in 0..MAX {
            s += "foo";
        }
        println!(
            "cost: {:?} ms, len: {}, cap: {}",
            t1.elapsed().as_millis(),
            s.len(),
            s.capacity()
        );
    }

    {
        let a = 1_000_i32;

        let s = a.to_string();
        assert_eq!(s, "1000");

        let a: i32 = s.parse().unwrap();
        assert_eq!(a, 1000_i32);

        let a: Result<i32, _> = "hello".parse();
        assert!(a.is_err());
    }

    {
        let hello = "hello,おはよう".to_string();

        // 只轉 ASCII 字元
        assert_eq!(hello.to_ascii_uppercase(), "HELLO,おはよう");
        assert_eq!(hello.to_ascii_uppercase().to_ascii_lowercase(), hello);

        // 如果非 ASCII 字元，也有大小寫時。
        let sigma = "Σ".to_string();
        assert_eq!(sigma.to_lowercase(), "σ");
        assert_eq!(sigma.to_lowercase().to_uppercase(), sigma);

        // 沒有大小寫的部分，不會變動。
        assert_eq!(hello.to_uppercase(), "HELLO,おはよう");
        assert_eq!(hello.to_uppercase().to_lowercase(), hello);

        // see https://doc.rust-lang.org/std/string/struct.String.html#method.to_lowercase
        let odysseus = "ὈΔΥΣΣΕΎΣ";
        assert_eq!("ὀδυσσεύς", odysseus.to_lowercase()); // but at the end of a word, it's ς, not σ:

        // see https://doc.rust-lang.org/std/string/struct.String.html#method.to_uppercase
        let s = "tschüß";
        assert_eq!("TSCHÜSS", s.to_uppercase()); // one character can become multiple

        // 修改原始字串中的 ascii 大小寫。
        let mut hello = "hello,おはよう".to_string();
        hello.make_ascii_uppercase();
        assert_eq!(hello, "HELLO,おはよう");
        hello.make_ascii_lowercase();
        assert_eq!(hello, "hello,おはよう");
    }

    {
        let s = "\n hello,\tworld !!! \t\n".to_string();
        assert_eq!(s.trim(), "hello,\tworld !!!");

        // unicode space: see https://zh.wikipedia.org/zh-tw/%E7%A9%BA%E6%A0%BC
        let s = "\u{0009}\u{0020}\u{00A0}\u{1680}hello\tworld!!!\u{2002}\u{2003}\u{2004}";
        assert_eq!(s.trim(), "hello\tworld!!!");

        // Text Direction Trim

        // Left to Right
        let s = "  English  ";
        assert!(Some('E') == s.trim_start().chars().next());
        assert!(Some('h') == s.trim_end().chars().rev().next());

        // Right to Left
        let s = "  עברית  ";
        println!("{:?}", s.trim_start());
        assert!(Some('ע') == s.trim_start().chars().next());

        println!("{:?}", s.trim_end());
        assert!(Some('ת') == s.trim_end().chars().rev().next());

        // Trim with predicate

        assert_eq!("111foo1bar11".trim_matches('1'), "foo1bar");
        assert_eq!("123foo1bar456".trim_matches(char::is_numeric), "foo1bar");
        assert_eq!(
            "123foo1bar456".trim_matches(|c: char| c.is_numeric()),
            "foo1bar"
        );

        assert_eq!("111foo1bar11".trim_start_matches('1'), "foo1bar11");
        assert_eq!(
            "123foo1bar456".trim_start_matches(char::is_numeric),
            "foo1bar456"
        );
        assert_eq!(
            "123foo1bar456".trim_start_matches(|c: char| c.is_numeric()),
            "foo1bar456"
        );

        assert_eq!("111foo1bar11".trim_end_matches('1'), "111foo1bar");
        assert_eq!(
            "123foo1bar456".trim_end_matches(char::is_numeric),
            "123foo1bar"
        );
        assert_eq!(
            "123foo1bar456".trim_end_matches(|c: char| c.is_numeric()),
            "123foo1bar"
        );

        assert_eq!("111foo1bar11".strip_prefix('1'), Some("11foo1bar11")); // 只移除一次
        assert_eq!(
            "111foo1bar11".strip_prefix(|c: char| c.is_whitespace()),
            None
        ); // 沒有移除任何東西，回傳 None

        assert_eq!("111foo1bar11".strip_suffix('1'), Some("111foo1bar1")); // 只移除一次
        assert_eq!(
            "111foo1bar11".strip_suffix(|c: char| c.is_whitespace()),
            None
        ); // 沒有移除任何東西，回傳 None
    }

    {
        assert!("hello\tworld!!!".contains('h'));
        assert!(!"hello\tworkd!!!".contains('z'));

        assert!("111foo1bar456".contains(['1', '2', 'z']));
        assert!(!"111foo1bar456".contains(['x', 'y', 'z']));

        assert!("111foo1bar456".contains(char::is_numeric));
        assert!(!"111foo1bar456".contains(char::is_control));

        assert!("111bar1bar456".contains("ar4"));
        assert!(!"111foo1bar456".contains("xyz"));

        assert_eq!("111foo1bar456".find(['4', '1', '2']), Some(0));
        assert_eq!("111foo1bar456".find("xyz"), None);

        assert_eq!("111foo1bar456".rfind(['4', '1', '2']), Some(10));
        assert_eq!("111foo1bar456".rfind("xyz"), None);

        assert_eq!("111foo1bar456".replace("111", "X"), "Xfoo1bar456");
        assert_eq!("111foo1bar456".replace("xyz", "abc"), "111foo1bar456");
        assert_eq!(
            "111foo1bar456".replace(char::is_numeric, "x"),
            "xxxfooxbarxxx"
        );

        assert_eq!(
            "111foo1bar456".replacen(['4', '1', '2'], "x", 1),
            "x11foo1bar456"
        );

        assert_eq!(
            "111foo1bar456".replacen(['4', '1', '2'], "x", 5),
            "xxxfooxbarx56"
        );
    }

    {
        assert_eq!(
            "123fooXYZbar!@#"
                .matches(['1', '4', '5'])
                .collect::<Vec<&str>>(),
            vec!["1"]
        );

        assert_eq!(
            "123fooXYZbar!@#"
                .matches(char::is_numeric)
                .collect::<Vec<&str>>(),
            vec!["1", "2", "3"]
        );

        assert_eq!(
            "123fooXYZbar!@#"
                .matches(char::is_uppercase)
                .collect::<Vec<&str>>(),
            vec!["X", "Y", "Z"]
        );

        assert_eq!(
            "123fooXYZbar!@#"
                .match_indices(char::is_uppercase)
                .collect::<Vec<_>>(),
            vec![(6, "X"), (7, "Y"), (8, "Z")]
        );
    }

    {
        let v: Vec<&str> = "abc def 123".split(' ').collect();
        let v1 = "abc def 123".split(' ').collect::<Vec<&str>>();
        assert_eq!(v, vec!["abc", "def", "123"]);
        assert_eq!(v, v1);

        let v = "2020-11-03 23:59"
            .split(['-', ' ', ':', '@'])
            .collect::<Vec<&str>>();
        let v1 = vec!["2020", "11", "03", "23", "59"];
        assert_eq!(v, v1);

        assert_eq!(
            "111foo1bar456"
                .split(char::is_numeric)
                .collect::<Vec<&str>>(),
            vec!["", "", "", "foo", "bar", "", "", ""]
        );

        assert_eq!("xyx=abc=def".split_once('='), Some(("xyx", "abc=def")));
        assert_eq!(
            "xyz=abc=def".splitn(2, '=').collect::<Vec<&str>>(),
            vec!["xyz", "abc=def"]
        );

        assert_eq!("xyz=abc=def".rsplit_once('='), Some(("xyz=abc", "def")));
        assert_eq!(
            "xyz=abc=def".rsplitn(2, '=').collect::<Vec<&str>>(),
            vec!["def", "xyz=abc"]
        );
    }

    {
        let mut a = 10;
        let mut c = 20;
        //let aa = &a;
        a = a + 1000;
        let mut bb = &mut a;
        println!("{:?}", bb);
        //*bb += 10;
        bb = &mut c;
        println!("{:?}", bb);
        *bb = 30;

        //assert_eq!(*aa, 20);
    }

    {
        let v = vec!["abc", "def", "xyz"];
        let r = v.join("-");
        assert_eq!(r, "abc-def-xyz".to_string());

        let v = vec!["abc".to_string(), "def".to_string(), "xyz".to_string()];
        let r = v.join("-");
        assert_eq!(r, "abc-def-xyz");

        let v = vec!["abc", "def", "123"];
        assert_eq!(v.join(" "), "abc def 123");

        let v = vec!["2020", "11", "03", "23", "59"];
        assert_eq!(v.join("-"), "2020-11-03-23-59");

        let v = vec!["", "", "", "foo", "bar", "", "", ""];
        assert_eq!(v.join(" "), "   foo bar   ");

        let v = vec!["", "", "", "foo", "bar", "", "", ""];
        assert_eq!(v.join(""), "foobar");
    }

    {
        let text = "foo\r\nbar\n\nbaz\r";

        let mut lines = text.lines();
        assert_eq!(lines.next(), Some("foo"));
        assert_eq!(lines.next(), Some("bar"));
        assert_eq!(lines.next(), Some(""));
        assert_eq!(lines.next(), Some("baz\r")); // Trailing carriage return is included in the last line
        assert_eq!(lines.next(), None);

        let text = "foo\r\nbar\n\nbaz\r";
        let lines = text.lines().collect::<Vec<&str>>();
        assert_eq!(lines, vec!["foo", "bar", "", "baz\r"]);
    }

    {
        let hello = "おはよう";

        hello.encode_utf16().for_each(|c| print!("{:X}, ", c)); // 304A, 306F, 3088, 3046,
        println!();
        hello
            .encode_utf16()
            .collect::<Vec<u16>>()
            .iter()
            .for_each(|c| print!("{:X}, ", c)); // 304A, 306F, 3088, 3046,

        println!();
        println!("{}", hello.len()); // 12
        println!("{}", hello.encode_utf16().count()); // 4
    }
}
