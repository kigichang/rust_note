# Rust Memory Principles

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [Ownership, Clone, Copy and Move](#ownership-clone-copy-and-move)
- [References and Borrowing](#references-and-borrowing)
  - [Reference and Dereference](#reference-and-dereference)
  - [Immutable Reference and Mutable Reference](#immutable-reference-and-mutable-reference)
  - [Borrowing](#borrowing)
- [Dangling Reference and Lifetimes](#dangling-reference-and-lifetimes)
  - [Lifetime Annotation](#lifetime-annotation)
  - [Lifetime Intersection](#lifetime-intersection)
  - [Lifetime Elision Rules](#lifetime-elision-rules)
  - [The Static Lifetime](#the-static-lifetime)
- [Summary](#summary)

<!-- /code_chunk_output -->

## Ownership, Clone, Copy and Move

Ownership 是 Rust 最核心與最重要的特性。簡單來說，擁有者是負責釋放記憶體的角色。當擁有者離開作用域或不再被使用時，需負責釋放記憶體。當擁有者的所有權轉移時，則原擁有者不能再被使用。Rust 的編譯器會在編譯時檢查是否使用到已喪失所有權的變數。Rust 是為了避免同一區塊的記憶體，被 Double Free 或 Memory Leak 的問題。

在 Rust 的 Ownership 觀念，主要有三個要點：

1. __Clone__: `Clone` 本身也是 Rust 一種 Trait，主要用來實作複制資料使用。Rust 原生的數值型別都有實作 `Clone` 特性。
1. __Copy__: `Copy` 本身也是 Rust 一種 Trait。在 Rust 中，常會與 `Clone` 搞混。簡單來說，當某個資料型別實作了 `Copy` 特性時，原變數不會喪失所有權；新的變數，會取得 __Clone__ 後的資料。因此，有`Copy` 特性的資料型別，前題是必須實作 `Clone` 特性。Rust 原生的數值型別，都有實作 `Copy` 特性。
    - `Copy` 的特性，跟 __Copy by Value__ 的概念相同，當賦值或傳入函式時，會複製一份資料。
1. Move: 如果某個資料型別沒有實作 `Copy` 特性，則當資料型別被賦值給其他變數，或傳入給函式時，原變數會失去所有權。這種特性稱為 Move。

```rust {.line-numbers}
let a = 10;
let b = a; // Copy

assert_eq!(a, 10);
assert_eq!(b, 10);

let a = format!("{:p}", &a); // 取得記憶體位置
let b = format!("{:p}", &b); // 取得記憶體位置
println!("{:?}", a);
println!("{:?}", b);
assert_ne!(a, b); // 不同記憶體位置

let s1 = "hello".to_string();
let s2 = s1; // Move
assert_eq!(s2, "hello");
println!("{:?}", s1); // Error
```

1. 數值類都都有實作 `Copy` 特性，因此在 `let b = a;` 時，`a` 並不會失去所有權。
1. `String` 類型沒有實作 `Copy` 特性，因此在 `let s2 = s1;` 時，`s1` 會失去所有權。也因此在 `println!("{:?}", s1);` 時，會出現編譯錯誤。
1. Rust 編譯器會在編譯時，檢查有沒有使用到失去所有權的變數。如果有，則會出現編譯錯誤。
1. Rust 編譯器另一個強項是，當有發生錯誤時，會說明原因，並提供建議修正的方式。

Rust 中，如果資料型別沒有 `Copy` 特性時，會經常發生所有權轉移。如：

1. 賦值給其他變數

    ```rust {.line-numbers}
    let s1 = "hello".to_string();
    let s2 = s1;
    println!("{:?}", s1); // Error
    ```

1. 傳入函數參數

    ```rust {.line-numbers}
    fn show(s: String) {
        println!("{}", s);
    }

    let s = "hello".to_string();
    show(s);
    println!("{:?}", s); // Error
    ```

1. 被移入 Closure (Closure 會在後面章節介紹)

    ```rust {.line-numbers}
    let hello = "hello".to_string();
    let my_print = || {
        println!("{}", hello);
        hello
    };

    my_print();
    println!("{:?}", hello);
    ```

1. 解構取值

    ```rust {.line-numbers}
    let a = Some(10);
    if let Some(x) = a {
        // Copy
        println!("is some of {}", x);
    } else {
        println!("is none");
    }
    println!("{:?}", a);

    let a = Some("abc".to_string());

    if let Some(ref x) = a {
        // 解構取出 Inner Reference
        assert_eq!(x, "abc");
    } else {
        assert!(false);
    }
    println!("{:?}", a); // 沒事

    if let Some(x) = a {
        // 解構取值
        assert_eq!(x, "abc");
    } else {
        assert!(false);
    }
    println!("{:?}", a); // Error
    ```

1. Into Interation (後面會提到)

    ```rust {.line-numbers}
    let v1 = vec![1, 2, 3];

    for x in &v1 {
        println!("{}", x);
    }

    println!("{:?}", v1); // Error

    for x in v1 {
        println!("{}", x);
    }
    println!("{:?}", v1); // Error
    ```

## References and Borrowing

Rust 提供了 Reference (參考) 以及 Borrowing (借用) 的機制，可以避免所有權轉移的問題。Reference 類似 C 的 Pointer 指向某個記憶體位置。Borrowing 則是指使用 Reference 的過程。

總之，使用 Reference 就不會有 Move 行為發生。

### Reference and Dereference

取得 Reference 的方式，與 C 相同是使用 `&` 符號。取得 Reference 後，可以使用 `*` 符號，取得 Reference 所指向的值。

```rust {.line-numbers}
let a = 10;
let aa = &a;
assert_eq!(*aa, 10);
```

### Immutable Reference and Mutable Reference

依變數是否為可變，分為兩種 Reference：

1. Immutable Reference: 使用 `&` 符號取得 Reference，可以同時取得多個 Immutable Reference。

    ```rust {.line-numbers}
    // 可以有多個 immutable reference
            let a = 10;
            let a1 = &a;
            let a2 = &a;

            assert_eq!(*a1, a);
            assert_eq!(*a2, a);
    ```

1. Mutable Reference: 使用 `&mut` 符號取得 Reference，當變數為可變時(mut)，才能取得Mutable Reference，且當 Mutable Reference 存在時，原變數不能再被使用。

    ```rust {.line-numbers}
    let mut a = 10;
    let aa = &mut a;
    *aa = *aa * 2;
    assert_eq!(a, 20); // Error, a 不可使用
    println!("{}", *aa);
    println!("{}", a);
    ```

    在以上範例中，修改成如下，則 `a` 可以使用。

    ```rust {.line-numbers}
    let mut a = 10;
    {
        let aa = &mut a;
        *aa = *aa * 2;
    }
    assert_eq!(a, 20);
    ```

    因為 `aa` 的生命週期，只在 `{}` 內，因此 `a` 可以使用。

    或者

    ```rust {.line-numbers}
    let mut a = 10;
    let aa = &mut a;
    *aa = *aa * 2;
    assert_eq!(a, 20);
    //println!("{}", *aa);
    println!("{}", a);
    ```

    因為 Rust 的編譯器，發現 `aa` 不再被使用，提早結果 `aa` 的生命週期，因此 `a` 可以使用。

### Borrowing

Borrowing 是指使用 Reference 的過程。在 Rust 中，Borrowing 會依變數是否為可變，分為兩種：

不可變借用與可變借用的使用規則如下：

1. 在同一個作用域中，一個記憶體可以有多个不可變借用（&T），且有不可變借用後，就不可以有可變借用 (&mut T)。

    ```rust {.line-numbers}
    let mut a = 10;
    let a1 = &a;
    let a2 = &mut a; // Error

    *a2 = *a1 * 2;
    println!("{}", a1);
    ```

1. 在同一個作用域，或者編譯器沒有釋放可變借用(&mut T)時，一個記憶體只有一個可變借用，且有可變借用後，就不能有不可變借用，也就是原變數無法再使用。

    ```rust {.line-numbers}
    let mut a = 10;
    let a2 = &mut a;
    *a2 = *a2 * 2;
    let a1 = &a; // Error
    let a3 = &mut a; // Error

    println!("{}", a2);
    ```

1. 編譯器檢查到不再使用借用時，會立即釋放。
1. 借用關係，在離開作用域後會釋放。

為什麼 Rust 的 Borrowing 會這麼嚴格？ Rust 為了避免記憶體同時或在多處被修改，而造成程式上操作錯誤。

## Dangling Reference and Lifetimes

Dangling Reference 與 C 相同，都是指使用到一處已被釋放或不存在的記憶體位置。在 Rust 中，編譯器會在編譯時期，檢查每個 Reference Lifetime，判斷是否有使用到 Dangling Reference。如果有，則會出現編譯錯誤。Reference 的生命週期，通常編譯器會自行推斷。如果編譯器無法推斷時，則需要使用 Lifetime Annotation (生命週期標註)，來標註 Reference 的生命週期，提供編譯器驗證。

```rust {.line-numbers}
let s;
{
    let s1 = "hello".to_string();
    s = &s1; //`s1` does not live long enough
}
println!("{}", s);
```

以上範例，`s1` 的生命週期，只在 `{}` 內，`s` 使用到已經被釋放的記憶體，因此會出現編譯錯誤。
用 Rust 的好處是使用 Rust 的設計(也可以說是限制)，就可以在編譯時期，找出這種問題。

### Lifetime Annotation

當 Rust 編譯器無法推斷 Reference 的生命週期時，則需要使用 Lifetime Annotation。在 Rust 中，Lifetime Annotation 使用 `'` 符號，後面接上自定義的 Lifetime 名稱，通常是由 `a` 開始命名，如：`'a`, `'b`。

```rust {.line-numbers}
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

let s1 = "hello".to_string();
let s2 = "abc".to_string();
let result = longest(&s1, &s2);
println!("{}", result);
```

以上的範例，`longest` 函式的回傳值，是 `x` 或 `y` 的 Reference，但編譯器無法推斷 `x` 或 `y` 的生命週期，因此會出現編譯錯誤。

```shell
150 |         fn longest(x: &str, y: &str) -> &str {
    |                       ----     ----     ^ expected named lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
    |
150 |         fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    |                   ++++     ++          ++          ++
```

Rust 的編譯器會提示，需要以及如何使用 Lifetime Annotation。這是 Rust 在縮短開發者的學習曲線，以及提高程式碼的可讀性上的努力。

`longest` 函式的修正後範例：

```rust {.line-numbers}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
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
```

1. `'a` 為 Lifetime Annotation，用來標註 `x` 與 `y` 的生命週期。
1. 生命週期 `'a` 用法，並非是指定 `x` 與 `y` 有相同的生命週期，初學者常會在此誤解。
1. 生命週期 `'a` 可以理解是 `x` 與 `y` 在程式邏輯上是有關聯的，而 `'a` 是取 `x` 與 `y` 的交集。
1. 在 `longest` 函式中，回傳值的生命週期，會依賴於 `x` 與 `y` 的生命週期。
1. 上例中的生命週期：`s1` > `s2` > `result`。

### Lifetime Intersection

在 Rust 中，當有多個生命週期時，則需明確指出交集。如下例：

```rust {.line-numbers}
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
```

`longest` 也可以寫成如下：

```rust {.line-numbers}
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
```

1. 以上 Lifetime Annotation 的寫法，其實是 Rust Generic 的語法，在 Rust 中，Lifetime Annotation 也是一種 Generic。
1. 其中 `'a: 'c` 在 Generic 上是指 `'a` 繼承自 `'c`。
1. 在繼承觀念上， `'a: 'c`, `'b: 'c` 代表 `'a` 與 `'b` 都是繼承自 `'c`，換句話說，`'c` 是不同型別 `'a` 與 `'b` 的交集。

- example from: [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- 有關 Gereric 的 Type Bound 可以參考 Scala 文件上的 [Upper Type Bounds](https://docs.scala-lang.org/tour/upper-type-bounds.html) 與 [Lower Type Bounds](https://docs.scala-lang.org/tour/lower-type-bounds.html)。

### Lifetime Elision Rules

在早期 Rust 版本，宣告 function 或 method 都需要明確指出 Lifetime Annotation。但在後來的版本，Rust 編譯器會自行推斷，如果可以推斷出來，則不需要明確指出。Rust 自行推斷的規則如下：

1. 為每個 Reference 參數，給予一個 Life Annotation。

    ```rust {.line-numbers}
    fn foo<'a>(x: &'a i32){}

    fn foo<'a, 'b>(x: &'a i32, y: &'b i32){}
    ```

1. 如果只有一個 Reference 參數，則回傳值的生命週期，會與參數的生命週期相同。

    ```rust {.line-numbers}
    fn foo<'a>(x: &'a i32) -> &'a i32 {}
    ```

1. 如果有多個輸入生命週期參數，但其中一個是 `&self` 或 `&mut self`，由於這是 Method (後面會提到)，所有輸出的 Reference 生命週期就會預設與 `self` 相同。

以前面的 `fn longest(x: &str, y: &str) -> &str` 為例，編譯器會自行推斷為 `fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str`，但無法推斷回傳值的生命週期，因此需要明確指出。

- 說明摘自 [透過生命週期驗證參考](https://rust-lang.tw/book-tw/ch10-03-lifetime-syntax.html)

### The Static Lifetime

在 Rust 中，有一個特殊的生命週期，稱為 `'static`。`'static` 生命週期是指整個程式的生命週期，通常用在全域變數上。String Literal 都有 `'static` 生命週期 (__All string literals have the 'static lifetime__)，如下：

```rust {.line-numbers}
let s: &'static str = "hello";
```

## Summary

因為記憶體安全的問題，Rust 的記憶體操作上，相對於其他程式語言，尤其是 GC 類的程式語言，會更為嚴格。初學者最容易在此階段放棄。建議先從簡單工具類的小程式先開始練習 Rust，不要一開始就寫有關資料結構的問題或者大型專案。從小工具的實戰經驗中，去體會 Rust 的記憶體管理特性，以及比較原先熟悉的程式語言，有什麼不同。

由於 Move, Borrowing, 及 Lifetimes 的特性，會與原 GC 類的程式語言實作方式，有很大的差異，造成用 Rust 實戰時，會處處卡關。以下分享我在實戰上的經驗與實作方式，讓初學者可以更快上手。

- 縮短變數的生命週期，提早釋放記憶體。
    1. 多利用 __變數 Shadowing__ 的特性，縮短已不用的變數生命週期，提早釋放記憶體。
    1. 多利用 __Scope (作用域)與 Scope 可回傳值__ 的特性，儘量縮短變數的生命週期。包含 Mutable Borrowing。如此可以避開 Borrowing 的限制。

- 變數儘可能為 __Immutable__，避免 Mutable Borrowing 的問題
    1. 以 Functional Programming 的觀念，變數為 immutable，可以避免同一個變數被多個函式同時修改的問題，提高程式碼的穩定度。
    1. 如果一個變數，大多數時間為 immutable，只有少數時間需要修改，可以使用 `Cell` 或 `RefCell` 來達成；如果是多執行緒的情況，則使用 `Mutex` 或 `RwLock`。

- 所有權控制
    1. 函式 `fn` 可以利用 return 值，來返還所有權。
    1. 模仿 Scala 的作法，利用 `Copy` 特性，避開 Ownership 與 Lifetime 的限制；但會因此多使用記憶體。
    1. Lifetime 基本上 Rust 編譯器會自行嚴格推斷，因此如果誤用時，修改程式即可。

- 共享或長期使用的記憶體
    1. 全域變數，可以利用 `static`, `once_cell` 或 `lazy_static` 達成。
    1. 使用 `Rc` 或 `Arc` 來達成 GC 的 Reference Counting 的特性。

Memo:

- `Cell`: 情境如 `let mut x = xxx;` 修改變數指向記憶體。
- `RefCell`: 情境 `let x = &mut x;` 沒有修改變數指向記憶體，但修改記憶體內容。
- `Rc` 是用在非多執行緒的情境，`Arc` 是用在多執行緒的情境。
