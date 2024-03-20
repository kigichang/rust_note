//use std::vec;

fn main() {
    //let a = 10;
    //println!("{}", a);
    //let a = a.to_string();
    //println!("{}", a);
    //println!("{}", (-8.0_f32).abs());
    //println!("Hello, world!");
    //
    //let x = 1_000_i32;
    //let y = x as i8;
    //println!("{} {}", x, y);
    //
    //let x = 136_i16;
    //let y = x as i8;
    //println!("{} {}", x, y);
    //
    //let x: u8 = 0b1000_1000;
    //println!("{}", x as i8);
    //
    //let arr = [0_i32; 10]; // 宣告陣列，給予初始值與長度
    //println!("{:?}", arr);
    //
    //let arr = [1, 2, 3, 4, 5]; // 宣告陳列
    //println!("{:?}", arr);
    //println!("{}", arr.len()); // 陣列長度
    //
    //println!("{}", arr[0]); // 取得陣列元素
    //
    //let arr2 = arr.map(|x| x * 2); // map 操作
    //println!("{:?}", arr);
    //println!("{:?}", arr2);
    //
    //let slice = &arr[1..3]; // 取得陣列切片 (slice)
    //println!("{:?}", slice);
    //let slice = &arr[1..=3]; // 取得陣列切片 (slice)
    //println!("{:?}", slice);
    //
    //let mut vec = vec![]; // 宣告 empty vector
    //vec.push(1_i32); // add element
    //vec.push(2_i32); // add element
    //
    //println!("{:?}", vec);
    //assert_eq!(vec.get(0), Some(&1_i32)); // get element
    //assert_eq!(vec[0], 1); // get element by index
    //
    //let mut vec = vec![1, 2, 3, 4, 5]; // 宣告 vector
    //vec.push(6); // add element
    //vec.push(7); // add element
    //println!("{:?}", vec);
    //assert_eq!(vec.get(vec.len() - 1), Some(&7_i32)); // get element
    //
    //for i in 0..vec.len() {
    //    println!("{}", vec[i]);
    //}
    //
    //let mut vec = vec![0_i32; 10]; // 宣告 vector，給予初始值與長度
    //for i in 0..vec.len() {
    //    vec[i] = i as i32;
    //}
    //println!("{:?}", vec);
    //
    //let slice = &vec[2..5]; // get slice of vector
    //println!("{:?}", slice);
    //
    //for i in 0..slice.len() {
    //    println!("{}", slice[i]);
    //}

    //{
    //    let hello = [
    //        b'h', b'e', b'l', b'l', b'o', b',', b'w', b'o', b'r', b'l', b'd',
    //    ];
    //
    //    let h1 = &hello[1..3];
    //    let h2 = &hello[6..];
    //
    //    println!("{}, {:?}, {:?}", hello.len(), hello.as_ptr(), hello);
    //    println!("{}, {:?}, {:?}", h1.len(), h1.as_ptr(), h1);
    //    println!("{}, {:?}, {:?}", h2.len(), h2.as_ptr(), h2);
    //}

    //{
    //    let hello = "hello,world".to_string();
    //    let h1 = &hello[1..3];
    //    let h2 = &hello[6..];
    //    println!("{:-2}, {:?}, {:?}", hello.len(), hello.as_ptr(), hello); // 11, 0x156606e10, "hello,world"
    //    println!("{:-2}, {:?}, {:?}", h1.len(), h1.as_ptr(), h1); //  2, 0x156606e11, "el"
    //    println!("{:-2}, {:?}, {:?}", h2.len(), h2.as_ptr(), h2); //  5, 0x156606e16, "world"
    //}
    //
    //{
    //    let おはよう = "おはよう".to_string();
    //
    //    let bytes = おはよう.bytes();
    //    for byte in bytes {
    //        print!("{:X} ", byte);
    //    }
    //    println!();
    //    let chars = おはよう.chars();
    //    for char in chars {
    //        print!("{:?}:{:X} ", char, char as u32);
    //    }
    //}

    //{
    //    let sparkle_heart = vec![240, 159, 146, 150];
    //
    //    let sparkle_heart = String::from_utf8_lossy(&sparkle_heart);
    //
    //    assert_eq!("💖", sparkle_heart);
    //
    //    // some invalid bytes
    //    let input = b"Hello \xF0\x90\x80World";
    //
    //    let output = String::from_utf8(input.to_vec());
    //    assert_ne!(output.is_ok(), true);
    //
    //    let output_with_replace = b"Hello \xEF\xBF\xBDWorld";
    //    let output = String::from_utf8_lossy(input);
    //    println!("from_utf8_lossy: {:?}", output);
    //    assert_eq!(output.as_bytes(), output_with_replace);
    //
    //    let output = unsafe { String::from_utf8_unchecked(input.to_vec()) };
    //    println!("from_utf8_unchecked: {:?}", output);
    //    assert_eq!(output.as_bytes(), input);
    //}

    //{
    //    let long_str = "hello,\
    //    world!\
    //    ";
    //    println!("{}, {}", long_str, long_str.len());
    //
    //    let long_str = "hello,
    //    world!
    //    ";
    //    println!("{}, {}", long_str, long_str.len());
    //
    //    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    //    println!("{}", raw_str);
    //
    //    // If you need quotes in a raw string, add a pair of #s
    //    let quotes = r#"And then I said: "There is no escape!""#;
    //    println!("{}", quotes);
    //
    //    // If you need "# in your string, just use more #s in the delimiter.
    //    // You can use up to 65535 #s.
    //    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    //    println!("{}", longer_delimiter);
    //}

    // {
    //     let おはよう = "hello,おはよう".to_string();

    //     let bytes = おはよう.bytes();
    //     for byte in bytes {
    //         print!("{:X} ", byte); // 68 65 6C 6C 6F 2C E3 81 8A E3 81 AF E3 82 88 E3 81 86
    //     }
    //     println!();

    //     let chars = おはよう.chars();
    //     for char in chars {
    //         print!("{:?}:{:X} ", char, char as u32); // 'h':68 'e':65 'l':6C 'l':6C 'o':6F ',':2C 'お':304A 'は':306F 'よ':3088 'う':3046
    //     }
    //     println!();

    //     let char_indices = おはよう.char_indices();
    //     for (idx, char) in char_indices {
    //         print!("{:?}:{}:{:X} ", idx, char, char as u32); // 0:h:68 1:e:65 2:l:6C 3:l:6C 4:o:6F 5:,:2C 6:お:304A 9:は:306F 12:よ:3088 15:う:3046
    //     }
    // }

    //{
    //    let text = "Some\nfancy\ntext\twith spaces, tabs, and newlines";
    //    let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
    //    println!("{:?}", words); // 輸出: ["Some", "fancy", "text", "with", "spaces,", "tabs,", "and", "newlines"]
    //}
}
