# Compound Data Types

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [前言](#前言)
- [Tuple](#tuple)
- [Array](#array)
- [Vector](#vector)
  - [產生 Vector](#產生-vector)
  - [修改 Vector](#修改-vector)
  - [Capcity](#capcity)

<!-- /code_chunk_output -->

## 前言

Rust 資料型別，其實與 Scala 非常相似。複合資料型態 (Compound Data Type) 有三種：

- Tuple
- Array
- Struct (後面會再說明)

## Tuple

Rust 與 Scala 都有完整 Tuple 設計，Go 在這方面相對缺乏。Tuple 是一種複合型別，可以包含不同型別的元素，並且 Tuple 的長度是固定的，即宣告後，無法再改變其長度。

```rust {.line-numbers}
fn main() {
    let x = (1_i32, true); // declare a tuple
    println!("{:?}", x); // (1, true)
    assert_eq!(x.0, 1_i32); // 取得 tuple 的第一個元素
    assert_eq!(x.1, true); // 取得 tuple 的第二個元素

    let (a, b) = x; // destructuring
    println!("a = {}, b = {}", a, b); // a = 1, b = true

    let x = 1_i32;
    let y = 2_i32;
    println!("x = {}, y = {}", x, y); // x = 1, y = 2
    let (y, x) = (x, y); // swap x and y
    println!("x = {}, y = {}", x, y); // x = 2, y = 1
}
```

Tuple 常用的情境：

1. 懶得宣告完整 struct，但又想要有 struct 的功能，如自定義 method。此項是 Go 沒有的。
1. 回傳多個值。Go 有支援，但會回傳兩個獨立的變數。沒有 struct 的特性。
1. 搭配解構 (destructuring)，可以快速交換兩個變數的值。

## Array

Array 只可包含相同型別的元素，並且 Array 的長度是固定的。在 Go 下，絕大部分的時間，都是在操作 Slice，並不會直接操作到 Array，因此會有 Slice 即 Array 的誤解。由於 Rust 的記憶體管理機制，Array 與 Slice 在實作上，需要特別留意。(Slice 後面會再說明)

```rust {.line-numbers}
fn main() {
    let arr = [0_i32; 10]; // 宣告陣列，給予初始值與長度
    println!("{:?}", arr); // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    let arr = [1, 2, 3, 4, 5]; // 宣告陳列
    println!("{:?}", arr); // [1, 2, 3, 4, 5]
    println!("length of arr = {}", arr.len()); // 陣列長度
    println!("first element = {}", arr[0]); // 取得陣列元素

    let arr2 = arr.map(|x| x * 2); // map 操作
    println!("{:?}", arr); // [1, 2, 3, 4, 5]
    println!("{:?}", arr2); // [2, 4, 6, 8, 10]

    let slice = &arr[1..3]; // 取得 Slice 的參照值
    println!("{:?}", slice); // [2, 3]

    let slice = &arr[1..=3]; // 取得 Slice 的參照值
    println!("{:?}", slice); //[2, 3, 4]
}
```

- Slice 的取法，與 Go 類似，可以使用 `..` 或 `..=` 來取得 Slice。

P.S. 在使用 `println!` 時，如果要印出 Array 或 Slice，可以使用 `{:?}` 來印出。

1. `{}` 是指資料型別有實作 `Display` trait。一般數值型別都有實作 `Display` trait。
1. `{:?}` 是指資料型別有實作 `Debug` trait。除自定義的 Struct、Enum，大部分的資料型別都有實作 `Debug` trait。

## Vector

Vector 是 Rust 最常用的資料型別之一，行為類似 Array，可以視作可變長度的 Array，提供了許多操作方法，如 `push`, `pop`, `insert`, `remove`, `splice`, `clear` 等等。

### 產生 Vector

```rust {.line-numbers}
let v = vec![1, 2, 3, 4, 5]; // v 在這裡被創建
assert!(!v.is_empty());

let v = vec![1; 5]; // 長度為 5 的向量，每個元素都是 1
assert_eq!(v, vec![1, 1, 1, 1, 1]);

let v: Vec<i32> = Vec::new(); // 空的 vector
assert_eq!(v.len(), 0);
assert_eq!(v.capacity(), 0);

let v = [1, 2, 3].to_vec();
assert_eq!(v.len(), 3);
assert_eq!(v.capacity(), 3);
assert!(!v.is_empty());

let v: Vec<i32> = Vec::with_capacity(10);
assert_eq!(v.len(), 0);
assert_eq!(v.capacity(), 10);
assert!(v.is_empty());

let v = vec!["a", "b", "c"];
assert_eq!(v[0], "a");
assert_eq!(v[2], "c");

let v: &[i32] = &(vec![1, 2, 3, 4, 5][1..4]); // 取得 Slice reference
assert_eq!(v, &[2, 3, 4]);
```

1. 使用 `vec!` macro，可以快速產生 Vector。
1. 使用 `Vec::new()`，可以產生空的 Vector。
1. 使用 `to_vec()`，可以將 Array 轉換成 Vector。

### 修改 Vector

```rust {.line-numbers}
let mut v = vec![1, 2, 3, 4, 5];
v.push(6);
assert_eq!(v, vec![1, 2, 3, 4, 5, 6]);
assert_eq!(v.pop(), Some(6));
assert_eq!(v, vec![1, 2, 3, 4, 5]);

v.insert(0, -1);
assert_eq!(v, [-1, 1, 2, 3, 4, 5]);
v.insert(v.len(), 6);
assert_eq!(v, [-1, 1, 2, 3, 4, 5, 6]);

assert_eq!(v.remove(0), -1);
assert_eq!(v, [1, 2, 3, 4, 5, 6]);
assert_eq!(v.remove(v.len() - 1), 6);
assert_eq!(v, [1, 2, 3, 4, 5]);

v[0] = 0;
assert_eq!(v, [0, 2, 3, 4, 5]);

v[0] = 1;
let new = vec![7, 8, 9];
let u = v.splice(1..=2, new).collect::<Vec<_>>();
assert_eq!(u, vec![2, 3]);
assert_eq!(v, vec![1, 7, 8, 9, 4, 5]);

v.clear();
assert!(v.is_empty());
```

- `push` 是將元素加入到 Vector 的尾端。
- `pop` 是將 Vector 的尾端元素移除。
- 結合 `push` 與 `pop` 操作，可以讓 Vector 作為 Stack 使用。
- `insert` 是將元素插入到指定的位置。
- `remove` 是將指定位置的元素移除。
- `splice` 是將指定範圍的元素移除，並且插入新的元素。
- `clear` 是將 Vector 清空。
- 可以使用 index 方式，來修改 Vector 的元素。

### Capcity

```rust {.line-numbers}
let v = vec![1, 2, 3, 4, 5];
println!("len: {}", v.len());
println!("cap: {}", v.capacity());

let mut v = Vec::new();
let mut cap = v.capacity();
for i in 0..100 {
    v.push(i);
    if cap != v.capacity() {
        println!("{}: cap: {} -> {}", i, cap, v.capacity());
        cap = v.capacity();
    }
}
println!("length = {}, capacity = {}", v.len(), v.capacity()); // length = 100, capacity = 128

println!("with capacity");
let mut v = Vec::with_capacity(100);
let mut cap = v.capacity();
for i in 0..100 {
    v.push(i);
    if cap != v.capacity() {
        println!("{}: cap: {} -> {}", i, cap, v.capacity());
        cap = v.capacity();
    }
}
println!("length = {}, capacity = {}", v.len(), v.capacity()); // length = 100, capacity = 100
```

- `len` 是 Vector 的長度。
- `capacity` 是 Vector 的容量。
- 當操作大量資料時，可以使用 `with_capacity` 來預先配置記憶體。以節省記憶體。
