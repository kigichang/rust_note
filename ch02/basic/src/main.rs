fn main() {
    // {
    //     let y: i32 = 5; // 指定型別 (integer 32 bits)
    //     let z = 5i32; // 指定型別 (integer 32 bits)
    //     let a = 5_u32; // 指定型別 (unsigned integer 32 bits)

    //     let x = 5; // 未指定型別
    //     let xx = 10_i64 + x; // 編譯器自動推斷 x 型別為 i64
    //     assert_eq!(x, 5_i64);
    //     assert_eq!(xx, 15_i64);

    //     let c = 'z';
    //     let z: char = 'ℤ'; // with explicit type annotation
    //     let heart_eyed_cat = '😻';
    // }

    // {
    //     let a = 98_222; // decimal 98,222
    //     assert_eq!(a, 98_222_i32);

    //     let b = 0xff; // hex 255
    //     let c = 0o77; // octal 63
    //     let d = 0b1111_0000; // binary 240
    //     let e = b'A'; // byte (u8 only)
    // }

    // {
    //     let _x = 5;
    //     let _y = 10;
    //     let _z = 15;
    // }

    // {
    //     let x = 5;
    //     println!("The value of x is: {}", x);
    //     x = 6; // 編譯錯誤
    //     println!("The value of x is: {}", x);
    // }

    // {
    //     let a = '😻';
    //     let b = a as i32;
    //     let c = b as i16;
    //     let d = c as i8;

    //     println!("({}) -> ({}) -> ({}) -> ({})", a, b, c, d); // (😻) -> (128571) -> (-2501) -> (59)

    //     let z = -357_i16;
    //     let x = z as i8;
    //     let y = z as u8;
    //     println!("{}", x); // -101
    //     println!("{}", y); // 155
    // }

    // {
    //     let x = 126_i8;
    //     let y = x.checked_add(10);
    //     assert_eq!(y, None);

    //     let z = x.checked_sub(10);
    //     assert_eq!(z, Some(116));

    //     let a = 2_u8;

    //     assert_eq!(a.checked_shl(1), Some(4)); // bit shift left
    //     assert_eq!(a.checked_shl(7), Some(0));
    //     assert_eq!(a.checked_shl(8), None);

    //     assert_eq!(a.checked_shr(1), Some(1)); // bit shift right
    //     assert_eq!(a.checked_shr(7), Some(0));
    //     assert_eq!(a.checked_shr(8), None);

    //     let x = 0b1000_1000;
    //     let y = x as i8;
    //     println!("{}", y); // -120
    // }

    // {
    //     let x = 126_i8;
    //     let y = x.wrapping_add(10); // x = 126 + 10 = 136, 136 % 2^8 = 0b1000_1000 & 0b_1111_1111 = 0b1000_1000 = -120
    //     assert_eq!(y, -120);

    //     let z = x.wrapping_sub(10);
    //     assert_eq!(z, 116);

    //     let a = 126_u8;
    //     let b = a.wrapping_add(200);
    //     assert_eq!(b, 70); // b % 2^8;

    //     let c = 2_u8;

    //     assert_eq!(c.wrapping_shl(1), 4); // bit shift left.
    //     assert_eq!(c.wrapping_shl(7), 0);
    //     assert_eq!(c.wrapping_shl(8), 2); // 8 mod 8 = 0
    //     assert_eq!(c.wrapping_shl(9), 4); // 9 mod 8 = 1, shift left 1 bit.

    //     assert_eq!(c.wrapping_shr(1), 1); // bit shift right.
    //     assert_eq!(c.wrapping_shr(7), 0);
    //     assert_eq!(c.wrapping_shr(8), 2); // 8 mod 8 = 0
    //     assert_eq!(c.wrapping_shr(9), 1); // 9 mod 8 = 1, shift right 1 bit.
    // }

    // {
    //     let x = 126_i8;
    //     let y = x.saturating_add(10); // max of i8 (2^7 -1)
    //     assert_eq!(y, 127);

    //     let z = (-128_i8).saturating_sub(127);
    //     assert_eq!(z, -128); // min of i8 (-2^7)
    // }

    // {
    //     let x = 126_i8;
    //     let y = x.overflowing_add(10);
    //     assert_eq!(y, (-120_i8, true));

    //     let z = x.overflowing_sub(10);
    //     assert_eq!(z, (116_i8, false));

    //     let a = 2_u8;

    //     assert_eq!(a.overflowing_shl(1), (4, false)); // bit shift left.
    //     assert_eq!(a.overflowing_shl(7), (0, false));
    //     assert_eq!(a.overflowing_shl(8), (2, true)); // 8 mod 8 = 0
    //     assert_eq!(a.overflowing_shl(9), (4, true)); // 9 mod 8 = 1, shift left 1 bit.

    //     assert_eq!(a.overflowing_shr(1), (1, false)); // bit shift right.
    //     assert_eq!(a.overflowing_shr(7), (0, false));
    //     assert_eq!(a.overflowing_shr(8), (2, true)); // 8 mod 8 = 0
    //     assert_eq!(a.overflowing_shr(9), (1, true)); // 9 mod 8 = 1, shift right 1 bit.
    // }

    // {
    //     let x: i32;
    //     println!("The value of x is: {}", x); // 編譯錯誤
    //     let x = 5;
    //     println!("The value of x is: {}", x);
    // }

    // {
    //     let x;

    //     // 在 scope 中，計算 x 的初始值
    //     {
    //         let a = 10_i32;
    //         x = a * 2;
    //     }
    //     assert_eq!(x, 20_i32); // 20_i32

    //     // 計算後，回傳並給定 x 的初始值
    //     let x = {
    //         let a = 10_i32;
    //         a * 2
    //     };
    //     assert_eq!(x, 20_i32); // 20_i32

    //     // 回傳 Unit 型別
    //     let x = {
    //         let a = 10_i32;
    //         a * 2;
    //     };
    //     assert_eq!(x, ()); // 與 scala 同，Unit 型別
    // }

    // {
    //     let mut x = 10;

    //     {
    //         let x = x + 1; // 使用 shadowing，將 x 改成 immutable.

    //         x = x * 2; // 編譯失敗
    //         println!("x in {{}} = {}", x);
    //     }

    //     x = x * 3;

    //     println!("x = {}", x);
    // }
}
