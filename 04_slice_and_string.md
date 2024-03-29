# Slice and String

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [Go Slice](#go-slice)
- [Rust Slice](#rust-slice)
- [Rust string, str, and &str](#rust-string-str-and-str)
  - [str](#str)
  - [UTF-8](#utf-8)
  - [Long and Raw String](#long-and-raw-string)
  - [String and str](#string-and-str)
  - [Byte and Char in String](#byte-and-char-in-string)
  - [常用操作](#常用操作)

<!-- /code_chunk_output -->

Rust 與 Go 在 Slice 使用上，有很大的差異。因此由 Go 轉換到 Rust 時，需要多比較兩者的差別。

## Go Slice

以下是 Go 的 Slice 記憶體結構。在 SliceHeader 中，有 `Len` 記錄 Slice 長度，`Cap` 記錄 Slice 容量，`Data` 記錄 Slice 資料的記憶體位址。

Go: SliceHeader(@deprecated)

```go
type SliceHeader struct {
 Data uintptr
 Len  int
 Cap  int
}
```

```go {.line-numbers}
package main

import (
    "fmt"
    "unsafe"
)

func main() {
    hello := make([]byte, 0, 15)
    hello = append(hello, []byte("hello,world")...)

    h1 := hello[1:3]
    h2 := hello[6:]

    //sh1 := (*reflect.SliceHeader)(unsafe.Pointer(&h1))

    fmt.Printf("%-2d %-2d %p %v\n", len(hello), cap(hello), unsafe.SliceData(hello), hello) // 11 15 0x14000110020 [104 101 108 108 111 44 119 111 114 108 100]
    fmt.Printf("%-2d %-2d %p %v\n", len(h1), cap(h1), unsafe.SliceData(h1), h1)             // 2  14 0x14000110021 [101 108]
    fmt.Printf("%-2d %-2d %p %v\n", len(h2), cap(h2), unsafe.SliceData(h2), h2)             // 5  9  0x14000110026 [119 111 114 108 100]
}
```

```ditaa
@startditaa
+--------+ +--------+ +--------+
| hello  | | h1     | | h2     |
| Cap 15 | | Cap 14 | | Cap  9 |
| Len 11 | | Len 2  | | Len  5 |
| Data   | | Data   | | Data   |
+-+------+ +-+------+ +-+------+
  :          :          :
  |          |          |
+-+ +--------+          |
|   |                   |
v   v                   v
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
| h | e | l | l | o | , | w | o | r | l | d |   |   |   |   |
+---+---+---+---+---+---+---+---+---+---+---+---+---+---+---+
@endditaa
```

![go slice](https://www.plantuml.com/plantuml/dpng/fPAnYiCm38PtFyMt-svmk0ntK-0wzYKCCRGGafAQTD73LmXZ79gf_U633qjWMNYuvxZcyPfZTFwxn7EeJhYaQLf0K0r0qGvJ9_p76w57AFfJbLzhEAUP411CwQXARmsdc2CW7AeEwUjaH-hWZvfM7GWrhJg_Vt9JaNrhKza0vIKRjiXtiIqZ96KgBCgNyYY-blDHOS-7Tmzf7kqpF040)

1. Go 的 Slice 背後都有一個陣列(Array)。
1. 平時操作 Go 的 Slice，它的本質其實是 `SliceHeader`。只是 Go 不建議直接操作 `SliceHeader`，目前 `SliceHeader` 標示為 `@deprecated`。
1. `SliceHeader` 內的 `Data` 欄位，記錄 Slice 在所使用陣列內開始位置。
1. `Len` 與 `Cap` 會依據 Slice 在開始位置而改變。

## Rust Slice

在 Rust 設計中，Slice 的型別是 `[Type]`，比如以下的 `let hx = h[6..];`，`hx` 的型別是 `[u8]`，但在此時會出現編譯錯誤；因為 Rust 編譯器無法在編譯時期得知 `[u8]` 的記憶體大小，無法儲入 Stack。所以在 Rust 中，無法宣告一個 Slice 型別的變數，必須使用 Slice 的參照 `&[Type]`，如 `let h1 = &h[1..5];`。[^unknown_size]

1. __參照__ 好比 C or Go 的 __Pointer__，Pointer 型別的長度是固定的。
1. 無法宣告一個資料型別的記憶體用量是不固定的變數。也是就說無法宣告 `let x: [u8]`。

```rust {.line-numbers}
fn main() {
    let h = [
        b'h', b'e', b'l', b'l', b'o', b',', b'w', b'o', b'r', b'l', b'd',
    ];

    let h1 = &h[1..5];
    let h2 = &h[6..];
    //let hx = h[6..]; // the size for values of type `[u8]` cannot be known at compilation time

    println!("{:-2}, {:?}, {:?}", h.len(), h.as_ptr(), h);      // 11, 0x7ffddab3f2cd, [104, 101, 108, 108, 111, 44, 119, 111, 114, 108, 100]
    println!("{:-2}, {:?}, {:?}", h1.len(), h1.as_ptr(), h1);   //  4, 0x7ffddab3f2ce, [101, 108, 108, 111]
    println!("{:-2}, {:?}, {:?}", h2.len(), h2.as_ptr(), h2);   //  5, 0x7ffddab3f2d3, [119, 111, 114, 108, 100]
}
```

```ditaa
@startditaa
 +-----+             +-----+
 | &h1 |             | &h2 |
 +--+--+             +--+--+
    :                   :
    |                   |
    v<--- h[1..5] ->|   v<-- h[6..] ------->|
+---+---+---+---+---+---+---+---+---+---+---+
| h | e | l | l | o | , | w | o | r | l | d |
+---+---+---+---+---+---+---+---+---+---+---+
|<------------------- h ------------------->|
@endditaa
```

![rust slice](https://www.plantuml.com/plantuml/dpng/bP312W8X54NtViLPjR4H9gZ5CCJyHxGG7328Wabgypw-rqDYm4qTk2ZdybIS7YKk9LrAZ0xVVV2i-3g7iCaziYxDxH0RzUseEGSmq39O8xI8DSzHRoQV-X3sPxgZLAVg488QGxtpklmS9sI4MNEjkMkscbVTBzKd_Zvxx5h8DCxUFSstP7__1W00)

1. `h[1..5]` 與 `h[6..]` 的資料型別都是 `[u8]`，但長度不同。
1. 與 GO 類比，`h[1..5]` 與 `h[6..]` 是 `SliceHeader.Data` 指向的位置。
1. 在 Go，通常都是直接稱呼 `h[1..5]` 為 Slice，但實際上是操作是`SliceHeader`，這中間有個隱藏的轉換，因此在學 Rust 時，常會誤解為什麼不能直接操作 Slice，而是要取得 Slice 的參照。

## Rust string, str, and &str

### str

`str` 是 Rust 的字串 Slice，無法直接宣告使用，因此必須使用 `&str`。

```rust {.line-numbers}
fn main() {
    let h: str; // 編譯錯誤
    let h: &str = "hello,world";
}
```

### UTF-8

Rust 與 Go 相同，字串都是採用 UTF-8 編碼。產生 `String` 最簡單的方式是使用 `to_string()`，或是使用 `String::from()`。

```rust {.line-numbers}
fn main() {
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
```

如果有遇到不合法的 UTF-8 編碼時，可以用：

1. `String::from_utf8_lossy()`：會將不合法的 UTF-8 編碼，轉換成 `U+FFFD REPLACEMENT CHARACTER`。
1. `String::from_utf8_unchecked()`：不檢查 UTF-8 編碼，直接轉換成字串；但為 __unsafe__ 的操作。

```rust
fn main() {
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
```

### Long and Raw String

```rust
fn main() {
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
```

#### Long String

1. 可以直接在兩個 `"` 內換行，效果如同在 Go 中使用 ``` ` ```。
1. 在兩個 `"` 內，可以使用 `\` 連接多行字串，效果如同 `line1 + line2 + line3`。

#### Raw String

1. 使用 `r"` 開頭，`"` 結尾。反斜線 `\` 不會被轉換。
1. 如果需要使用 `"`，可以使用 `r#"` 開頭，`"#` 結尾。
1. 如果需要使用 `#`，利用多個 `#` 在開頭與結尾。如 `r###"` 開頭，`"###` 結尾。

### String and str

Rust 的字串，與 Go 的字串在設計上有很大的差異。在 Go `string` 操作上，比較像是 primitive type。Rust `String` 是物件導向的物件。

Rust 的字串中，常會看到三種型別：

1. `String`: 可變字串，可以視為 `Vec<u8>` 的集合。(`Vec` vector 後面會說明)
1. `str`: 字串 Slice
1. `&str`: 字串 Slice 的參照

`str` 與先前提到的 Slice 相同，因為是不固定長度，因此必須使用參照 `&str`。

```rust {.line-numbers}
fn main() {
    let hello: String = "hello,world".to_string();
    let h1: &str = &hello[1..5];
    let h2: &str = &hello[6..];
    println!("{:-2}, {:?}, {:?}", hello.len(), hello.as_ptr(), hello); // 11, 0x563edb2179d0, "hello,world"
    println!("{:-2}, {:?}, {:?}", h1.len(), h1.as_ptr(), h1); //  4, 0x563edb2179d1, "ello"
    println!("{:-2}, {:?}, {:?}", h2.len(), h2.as_ptr(), h2); //  5, 0x563edb2179d6, "world"
}
```

```ditaa
@startditaa

     +-------+    +-----+    +-----+
     | hello |    | &h1 |    | &h2 |
     +--+----+    +--+--+    +--+--+
        |            :          :
        |   +--------+          |
Stack   |   |                   |
--------|---|-------------------|-----------------------
Heap    |   |                   |
        |   v                   v
        V    <--- h[1..5] ->|    <-- h[6..] ------->|
        +---+---+---+---+---+---+---+---+---+---+---+
        | h | e | l | l | o | , | w | o | r | l | d |
        +---+---+---+---+---+---+---+---+---+---+---+
@endditaa
```

![Rust String, str, and &str](https://www.plantuml.com/plantuml/dpng/dP8n2y8m48Nt_8gRN6835NGGAHtT1HTn23OGCLYfeIxvyLxDeprGHFpWmSlZ-dA5rFTekzYSexLAqO0kCdewI9k74daNGaif7sQ-dEo2qjYavUTQM3IXJB0MLaxWVjI1f7RHdYw3WmIOA4229CW_ehREtkXhjLolftVwSM9Fp8PRoHzAOvP7Agg4ZAELCPnaAlJYbt-MsCQp72j0BMlEUi1to1lwzwxQNPlNeta2)

### Byte and Char in String

與 Go 相同，Rust 字串是 UTF-8 編碼。在 Rust 中，字串是 `Vec<u8>` 的集合，因此可以使用 `bytes()` 取得字串的 byte iterator，也可以使用 `chars()` 取得字串的 `char` iterator (Go `rune`) 。

```rust {.line-numbers}
fn main() {
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
```

1. 留意 `char_indices()` 的回傳值，是一個 tuple，第一個值是該字元以 __byte__ 為單位的索引值，並非以 __char__ 為單位的索引值，第二個值是字元本身。

### 常用操作

#### 建立字串與基本操作

```rust {.line-numbers}
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
```

- `len()`: 字串長度
- `capacity()`: 字串容量
- `is_empty()`: 是否為空字串
- `is_ascii()`: 是否為 ASCII 字串

#### 修改字串

```rust {.line-numbers}
let mut s = String::from("foo");
s.push_str("bar");
println!("{}", s); // foobar

s.push('!');
println!("{}", s); // foobar!

s += "!";
println!("{}", s); // foobar!!

s = format!("{}-{}", s, s);
println!("{}", s); // foobar!!-foobar!!
```

- `push_str()`: 附加字串，輸入參數是 `&str`
- `push()`: 附加字元，輸入參數是 `char`
- `+=`: 附加字串，輸入參數是 `&str`
- `format!()`: 來處理字串格式化，與 Go 的 `fmt.Sprintf()` 相同。

#### 大量附加字串

與 Java/Scala/Go 相同，都需要使用 `StringBuilder` 來附加字串，效能會比直接使用 `+` 來附加字串好。

```rust {.line-numbers}
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
```

結果：

```shell
0-idx realloc: 0x1 --> 0x6000024e0050 with new capacity 8
5-idx realloc: 0x6000024e0050 --> 0x6000026e51e0 with new capacity 32
10-idx realloc: 0x6000026e51e0 --> 0x6000033e4280 with new capacity 64
21-idx realloc: 0x6000033e4280 --> 0x6000008e4180 with new capacity 128
42-idx realloc: 0x6000008e4180 --> 0x600001ae4000 with new capacity 256
85-idx realloc: 0x600001ae4000 --> 0x120604830 with new capacity 512
```

由上例中，可以看到 Rust 在附加字串時，會重新配置記憶體，新加的容量為 8 bytes，爾後以指數成長方式增加。

```rust {.line-numbers}
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
```

結果：

```shell
cost: 158 ms, len: 30000000, cap: 33554432
cost: 154 ms, len: 30000000, cap: 30000000
```

由上結果，速度上，差異不大，但是在記憶體使用上，預先計算好需要的記憶體，並使用 `String::with_capacity()` 初始字串，在記憶體使用上會比較好。

#### 數字與字串轉換

```rust {.line-numbers}
let a = 1_000_i32;

let s = a.to_string();
assert_eq!(s, "1000");

let a: i32 = s.parse().unwrap();
assert_eq!(a, 1000_i32);

let a: Result<i32, _> = "hello".parse();
assert!(a.is_err());
```

#### byte 操作

```rust {.line-numbers}
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
```

- `as_bytes()`: 取得字串的 byte slice 參照.
- `as_bytes_mut()`: 取得字串的 byte slice 的可變參照。
- 如果 method 是 `unsafe` 時，需要使用 `unsafe` scope 來操作。

#### Slice 操作

```rust {.line-numbers}
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
```

- `get()`: 取得 `Option`, 內含字串的 slice 參照。
- `get_mut()`: 取得 `Option`，內含字串的 slice 的可變參照。
- 如果超過範圍，會回傳 `None`。

#### Lower and Upper Case

```rust {.line-numbers}
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
```

- `to_ascii_uppercase()`: 只轉 ASCII 字元為大寫。
- `to_ascii_lowercase()`: 只轉 ASCII 字元為小寫。
- `to_uppercase()`: Unicode 轉大寫，會依字元而定。
- `to_lowercase()`: Unicode 轉小寫，會依字元而定。
- `make_ascii_uppercase()`: 修改原始字串中的 ascii 轉成大寫。
- `make_ascii_lowercase()`: 修改原始字串中的 ascii 轉成小寫。

#### Trim

```rust {.line-numbers}
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
```

- `trim()`: 移除字串前後的空白字元。空白字元是指 [Unicode 定義的空白字元](https://zh.wikipedia.org/zh-tw/%E7%A9%BA%E6%A0%BC)。
- Text Direction Trim: `trim_start()` 與 `trim_end()`，分別是移除字串開頭與結尾的空白字元。會依照語系的文字方向而定。如英文是由左至右，希伯來文是由右至左。
- Trim with predicate: 可以使用 `trim_matches()`、`trim_start_matches()`、`trim_end_matches()`、`strip_prefix()`、`strip_suffix()` 來移除符合條件的字元。輸入的參數可以是：
  - `char`
  - [char]
  - function returns `bool`
  - closure returns `bool`
- `strip_prefix()` 與 `strip_suffix()` 只會移除一次符合條件的字元。並回傳 `Option`。如果沒有符合條件的字元，會回傳 `None`。

#### Containes, Find, and Replace

```rust
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
```

- `contains`: 回傳是否包含指定的字元、字串、或是符合條件的字元。
- `find`, `rfind`: 回傳第一個或最後一個符合條件的字元的索引值。與 Go 不同，Go 找不到時，會回傳 `-1`。Rust 回傳 `Option`。如果找不到時，則回傳 `None`。
- `replace`: 取代字串中的字元、字串、或是符合條件的字元。`replacen` 可以指定取代的次數。

#### Matches

```rust {.line-numbers}
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
```

- `matches`: 回傳符合條件的字元的字串 Slice 的 iterator。
- `match_indices`: 回傳符合條件的字元的索引值與字串 Slice 的 iterator。
- `collect::<Vec<&str>>()`: 將 iterator 轉換成 `Vec<&str>`。

#### Split

```rust {.line-numbers}
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
```

- `split`: 依照指定的字元、字串、或是符合條件的字元來分割字串。
- `split_once`: 只分割一次，回傳 `Option`。
- `splitn`: 可以指定分割的次數。
- `rsplit_once`: 由右至左分割一次，回傳 `Option`。
- `rsplitn`: 由右至左分割，可以指定分割的次數，留意 `rsplitn` 的回傳值是由右至左。

#### Join

```rust {.line-numbers}
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
```

- `join`: 將 `Vec` 中的字串，依照指定的字串來連接。回傳一個新的 `String`。

#### Lines

```rust {.line-numbers}
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
```

- `lines`: 將字串依照換行符號 `\n` 來分割，回傳一個字串 Slice 的 iterator。
- 如果 `\r` 後面沒有 `\n`，則 `\r` 會被當作字串的一部分。

#### To UTF16

```rust {.line-numbers}
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
```

- `encode_utf16`: 將字串轉換成 UTF-16。

[^unknown_size]: 在存入 Stack 的資料，都需要是固定 Size，在 Rust 與 Go 的 Slice 中，其實都是不固定的，因此在 Go 的設計中，使用 `SliceHeader` 來操作 Slice；在 Rust 中，使用 Slice 的參照來操作。
