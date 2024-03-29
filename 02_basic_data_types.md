# Basic Data Type

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [Scalar Data Types](#scalar-data-types)
  - [宣告變數](#宣告變數)
  - [Rust 與 Go 的差異](#rust-與-go-的差異)
- [Immutable & Mutable](#immutable--mutable)
- [數值間 Cast](#數值間-cast)
- [數值 Overflow 處理](#數值-overflow-處理)
- [Declaration, Scope, Shadowing and Freeze](#declaration-scope-shadowing-and-freeze)
  - [First Declaration](#first-declaration)
  - [Variable Shadowing](#variable-shadowing)
  - [Scope](#scope)
  - [Freeze Mutable](#freeze-mutable)

<!-- /code_chunk_output -->

## Scalar Data Types

與 Go 相同，提供 integer, float, boolean, char 等資料型別，也有 `+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `!`, `<<`, `>>` 等運算；但本質與使用上，差異頗大，Rust 的 Scalar Data Types 使用上，比較像 Scala 支援 method。Go 則是 primitive type。

以下是 Go 與 Rust 型別對照表：

| Go | Rust |
| - | - |
| int8 | i8 |
| int16 | i16 |
| int32 | i32 |
| int64 | i64 |
| - | i128 |
| uint8 | u8 |
| uint16 | u16 |
| uint32 | u32 |
| uint64 | u64 |
| - | u128 |
| int | isize |
| uint| usize |
| flat32 | f32 |
| flat64 | f64 |
| bool | bool |
| byte (uint8) | u8 |
| rune (int32) | char |

### 宣告變數

Rust 使用 `let` 關鍵字宣告變數，與 Go 的 `var` 類似。

```rust {.line-numbers}
let y: i32 = 5; // 指定型別 (integer 32 bits)
let z = 5i32; // 指定型別 (integer 32 bits)
let a = 5_u32; // 指定型別 (unsigned integer 32 bits)

let x = 5; // 未指定型別
let xx = 10_i64 + x; // 編譯器自動推斷 x 型別為 i64
assert_eq!(x, 5_i64);
assert_eq!(xx, 15_i64);

let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let heart_eyed_cat = '😻';
```

在數值宣告，Rust 也有支援較 friendly 的寫法，如下：

```rust
let a = 98_222; // decimal 98,222
let b = 0xff; // hex 255
let c = 0o77; // octal 63
let d = 0b1111_0000; // binary 240
let e = b'A'; // byte (u8 only)
```

### Rust 與 Go 的差異

| | Rust | Go |
| - | - | - |
| 推斷數值型別 | 編譯器自動推斷 | 數值預設是 int / float64  |
| 宣告變數 | `let x: isize = 5;` | `var x int = 5` |
| 宣告常數 | `const MAX_POINTS: u32 = 100_000;` | `const MaxPoints = 100_000` |
| 預設值 | 不支援，在使用變數前，必須宣告值，否則會 __編譯錯誤__。 | Zero Value |
| 宣告但不使用 | 會警告，可用 wildcard `_` 來忽略。 | 會編譯錯誤，可利用 `var _ = x` 來避開。 |
| 可變性 | 需用 `mut` 指定可變 | 除 `const` 外，預設可變 |
| 數值運算 | 有支援 overflow 檢查 | 不支援 overflow 檢查 |
| 方法(Method) | 支援. eg: `(-8.0_f64).abs()` | 不支援. `Math.Abs(-8.0)` |

當變數宣告後，沒有被使用時，Rust 會警告，可以在變數前加 __`_`__ wildcard 來處理，但 Go 會編譯錯誤，需用 `var _ = x` 來忽略。

```rust {.line-numbers}
let _x = 5;
let _y = 10;
let _z = 15;
```

## Immutable & Mutable

與 Scala (Functional Language 設計) 同，變數與函式(Function)/方法(Method)的參數都是 __Immutable__ ，若要變成 Mutable，需用 `mut` 關鍵字指定。

```rust {.line-numbers}
let x = 5;
println!("The value of x is: {}", x);
x = 6; // 編譯錯誤
println!("The value of x is: {}", x);
```

## 數值間 Cast

使用 `as` 做轉型。

```rust {.line-numbers}
let a = '😻';
let b = a as i32;
let c = b as i16;
let d = c as i8;

println!("({}) -> ({}) -> ({}) -> ({})", a, b, c, d); // (😻) -> (128571) -> (-2501) -> (59)

let z = -357_i16;
let x = z as i8;
let y = z as u8;
println!("{}", x); // -101
println!("{}", y); // 155
```

## 數值 Overflow 處理

Rust 提供檢查並處理 overflow 的機制，以下是各函式的前綴與說明：

- checked: 檢查是否有 overflow，回傳 Option。(Option 後面會再說明)

    ```rust {.line-numbers}
    let x = 126_i8;
    let y = x.checked_add(10);
    assert_eq!(y, None);

    let z = x.checked_sub(10);
    assert_eq!(z, Some(116));
    
    let a = 2_u8;
    
    assert_eq!(a.checked_shl(1), Some(4)); // bit shift left
    assert_eq!(a.checked_shl(7), Some(0));
    assert_eq!(a.checked_shl(8), None);
    
    assert_eq!(a.checked_shr(1), Some(1)); // bit shift right
    assert_eq!(a.checked_shr(7), Some(0)); 
    assert_eq!(a.checked_shr(8), None);
    ```

- warpping: 如果有 overflow，取該型別的餘數。如 i16，則取 2^16^ 的餘數。

    ```rust {.line-numbers}
    let x = 126_i8;
    let y = x.wrapping_add(10); // x = 126 + 10 = 136, 136 % 2^8 = 0b1000_1000 & 0b_1111_1111 = 0b1000_1000 = -120
    assert_eq!(y, -120);

    let z = x.wrapping_sub(10);
    assert_eq!(z, 116);

    let a = 126_u8;
    let b = a.wrapping_add(200);
    assert_eq!(b, 70); // b % 2^8;

    let c = 2_u8;

    assert_eq!(c.wrapping_shl(1), 4); // bit shift left.
    assert_eq!(c.wrapping_shl(7), 0);
    assert_eq!(c.wrapping_shl(8), 2); // 8 mod 8 = 0
    assert_eq!(c.wrapping_shl(9), 4); // 9 mod 8 = 1, shift left 1 bit.

    assert_eq!(c.wrapping_shr(1), 1); // bit shift right.
    assert_eq!(c.wrapping_shr(7), 0);
    assert_eq!(c.wrapping_shr(8), 2); // 8 mod 8 = 0
    assert_eq!(c.wrapping_shr(9), 1); // 9 mod 8 = 1, shift right 1 bit.
    ```

- saturating: 如果有 overflow，則回傳該型別的最大值或最小值。

    ```rust {.line-numbers}
    let x = 126_i8;
    let y = x.saturating_add(10); // max of i8 (2^7 -1)
    assert_eq!(y, 127);

    let z = (-128_i8).saturating_sub(127);
    assert_eq!(z, -128); // min of i8 (-2^7)
    ```

- overflowing: 回傳 tuple (tuple 後面會再說明)，第一個值為 __`wrapping`__ 的結果，第二個值為 boolean，表示是否有 overflow。

    ```rust {.line-numbers}
    let x = 126_i8;
    let y = x.overflowing_add(10);
    assert_eq!(y, (-120_i8, true));

    let z = x.overflowing_sub(10);
    assert_eq!(z, (116_i8, false));

    let a = 2_u8;

    assert_eq!(a.overflowing_shl(1), (4, false)); // bit shift left.
    assert_eq!(a.overflowing_shl(7), (0, false));
    assert_eq!(a.overflowing_shl(8), (2, true)); // 8 mod 8 = 0
    assert_eq!(a.overflowing_shl(9), (4, true)); // 9 mod 8 = 1, shift left 1 bit.

    assert_eq!(a.overflowing_shr(1), (1, false)); // bit shift right.
    assert_eq!(a.overflowing_shr(7), (0, false));
    assert_eq!(a.overflowing_shr(8), (2, true)); // 8 mod 8 = 0
    assert_eq!(a.overflowing_shr(9), (1, true)); // 9 mod 8 = 1, shift right 1 bit.
    ```

P.S. `assert_eq!` 是 Rust 的 macro，用來檢查值是否相等。另一個是 `assert_ne!`，用來檢查值是否不相等。

## Declaration, Scope, Shadowing and Freeze

由於 Rust 特殊的 ownership 的記憶體管理，因此在 scope 的使用時機上，就比其他程式語言更為重要。

### First Declaration

在變數使用前，一定要宣告其值，否則會 __編譯錯誤__。

```rust {.line-numbers}
let x:i32;
println!("The value of x is: {}", x); // 編譯錯誤
let x = 5;
println!("The value of x is: {}", x);
```

### Variable Shadowing

簡單來說，就是在同一個 scope 中，變數名稱可以重覆使用。

```rust {.line-numbers}
let x = 5;
let x = x + 1;
let x = x * 2;
println!("The value of x is: {}", x); // 12
```

### Scope

在 Rust 設計上，scope 是以 `{}` 來定義。

```rust {.line-numbers}
let x;

// 在 scope 中，計算 x 的初始值
{
    let a = 10_i32;
    x = a * 2;
}
assert_eq!(x, 20_i32); // 20_i32

// 計算後，回傳並給定 x 的初始值
let x = {
    let a = 10_i32;
    a * 2
};
assert_eq!(x, 20_i32); // 20_i32

// 回傳 Unit 型別
let x = {
    let a = 10_i32;
    a * 2;
};
assert_eq!(x, ()); // 與 scala 同，Unit 型別
```

1. 在 Rust 設計中，__`;`__ 代表 statement 的結束，並回傳 Unit 型別。因此，若在 scope 最後一行，加上 __`;`__，則會回傳 Unit 型別。如果不加 __`;`__，則會回傳該 scope 最後一行的值。
1. 此一設計，與 Scala 相同，在 Scala 設計中，有 `return` 關鍵字，但 Scala 不鼓勵使用，而是以最後一行的值，作為回傳值。
1. 在 Functional Lanaguage 設計上，Early Return 被視為 Side Effect，因此不鼓勵使用。

### Freeze Mutable

可以利用 scope 與 shadowing 的特性，達到 freeze mutable 的效果。

```rust {.line-numbers}
let mut x = 10;

{
    let x = x + 1; // 使用 shadowing，將 x 改成 immutable.

    x = x * 2; // 編譯失敗
    println!("x in {{}} = {}", x);
}

x = x * 3;

println!("x = {}", x);
```
