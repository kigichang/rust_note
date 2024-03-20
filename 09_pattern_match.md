# Patten Match

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [前言](#前言)
- [let and ref](#let-and-ref)
  - [if let](#if-let)
- [match](#match)

<!-- /code_chunk_output -->

## 前言

Pattern Match 可以看成一種超級強化的 `switch`，在 Rust 與 Scala 都有支援，Go 在這方面非常不足。我第一次使用 Pattern Match 功能，是在使用 Scala 時，就被這項功能給吸引。在使用 Go 時，就覺得沒有這項功能，非常不方便。

Pattern Match 的概念，主要有兩項：

1. Pattern: 用來比對是否為某種型別或值。
1. Match: 是否符合某項條件。

因此使用 Pattern Match 時，必須對資料型別進行解構 (Destructuring)，因此可能造成所有權的轉移，使用時要特別留意。在 Scala 還可定義解構行為。

Rust 的 Pattern Match 主要使用 `let` 與 `match` 這兩個關鍵字。

## let and ref

Rust `let` 可以用在宣告變數，也可以用在解構。在解構時，如果內部的資料型別，沒有 `Copy` 的特性時，則內部資料所有權，會轉移至新的變數上，造成被解構的變數，就不可再使用。
可以使用 `ref` 關鍵字，或者使用 `&` 在被解構的變數，來取得內部資料的參考，如此就不會因解構後，造成原變數無法使用。

```rust {.line-numbers}
let p = (1_i32, 2_i32);
let (x, y) = p;
println!("{:?}", x);
println!("{:?}", y);
println!("{:?}", p);
```

1. 使用 `let` 將 `p` 解構，並將內部的值，分別指定給 `x` 與 `y`。
1. 由於 i32 有 `Copy` 特性，因此 `p` 仍然可以使用。

```rust {.line-numbers}
let p = ("a".to_string(), "b".to_string());
let (x, y) = p;
println!("{:?}", x);
println!("{:?}", y);
println!("{:?}", p); // p is moved
```

1. 使用 `let` 將 `p` 解構，並將內部的值，分別指定給 `x` 與 `y`。
1. 由於 `String` 沒有 `Copy` 特性，因此 `p` 就無法再使用。

```rust {.line-numbers}
let p = ("a".to_string(), "b".to_string());
let (x, y) = &p;
println!("{:?}", x);
println!("{:?}", y);
println!("{:?}", p);
```

1. 使用 `let` 將 `&p` 解構，並將內部的值，分別指定給 `x` 與 `y`。
1. 此時 `x`, `y` 是取得 `p` 內部資料的參考，資料型別為 `&String`。
1. `p` 並沒有喪失內部資料的所有權，因此 `p` 仍然可以使用。

```rust {.line-numbers}
let p = ("a".to_string(), "b".to_string());
let (ref x, ref y) = p;
println!("{:?}", x);
println!("{:?}", y);
println!("{:?}", p);
```

1. 利用 `ref` 關鍵字，取得被解構變數的內部資料參考。
1. `let (ref x, ref y) = p;` 等同於 `let (x, y) = &p;`。
1. 此時 `x`, `y` 是取得 `p` 內部資料的參考，資料型別為 `&String`。
1. `p` 並沒有喪失內部資料的所有權，因此 `p` 仍然可以使用。

```rust {.line-numbers}
let mut p = ("a".to_string(), "b".to_string());
let (ref mut x, ref mut y) = p;

x.push_str("c");
y.push_str("d");

println!("{:?}", p);
```

1. 使用 `ret mut` 取得內部資料的可變參考。
1. 透過內部參考，修改內部資料。

### if let

`if` 與 `let` 結合，常用於 Enum 的型別，除了可以解構取得內部資料外，也可以針對變數，判斷屬於 Enum 那一種型別。使用  `if-let` 還可針對取得的內部資料，加上條件判斷。

`if-let` 功能如下：

1. 解構變數
1. 如果是 Enum 型別，可以判斷是否為某一種型別
1. 可以加上條件判斷

```rust {.line-numbers}
```

## match
