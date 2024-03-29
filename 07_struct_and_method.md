# Struct

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [前言](#前言)
- [Declare Struct](#declare-struct)
  - [Named Fields](#named-fields)
  - [Without Field](#without-field)
  - [Tuple Struct (Without Named Fields)](#tuple-struct-without-named-fields)
  - [Drive Debug](#drive-debug)
  - [println!("{:?}") and fmt::Debug](#println-and-fmtdebug)
- [Method](#method)
  - [Static Method](#static-method)
  - [Immutable and Mutable Reference in Method](#immutable-and-mutable-reference-in-method)
- [More about Ownership](#more-about-ownership)
- [Cell and RefCell](#cell-and-refcell)

<!-- /code_chunk_output -->

## 前言

Rust 的 Struct 與 C, Go 相同，都是可以包含多個不同型別的變數。並自定義方法 (Method)。
Rust 與 Go 相同，Struct 並沒有繼承。因此不算是物件導向語言。

## Declare Struct

Rust 使用方式有三種：

1. Named Fields: 最常見的使用方式。
1. Without Field: 不包含任何欄位。
1. Tuple Struct: 沒有名稱的欄位方式。

### Named Fields

最常見的使用方式，並使用欄位名稱來存取。使用方式與 Go 相同，沒有像 C 有區分 `.` 與 `->`，統一使用 `.`。

```rust {.line-numbers}
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 0 };

println!("({}, {})", p.x, p.y);
```

### Without Field

Rust 允許 Struct 不包含任何欄位。

```rust {.line-numbers}
struct Point;
let _p = Point;
```

### Tuple Struct (Without Named Fields)

Rust 允許 Struct 沒有名稱的欄位，以 Tuple 的方式宣告與存取。

```rust {.line-numbers}
struct Point(i32, i32);
let p = Point(0, 0);
println!("({}, {})", p.0, p.1);
```

### Drive Debug

在當要 `println!("{:?}")` struct 時，一定會發生編譯錯誤如下：

```rust {.line-numbers}
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 0 };

println!("({}, {})", p.x, p.y);
println!("{:?}", p);
```

編譯錯誤如下：

```shell
error[E0277]: `Point` doesn't implement `Debug`
  --> src/main.rs:26:26
   |
26 |         println!("{:?}", p);
   |                          ^ `Point` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Point`
   = note: add `#[derive(Debug)]` to `Point` or manually `impl Debug for Point`
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Point` with `#[derive(Debug)]`
   |
18 +         #[derive(Debug)]
19 |         struct Point {
   |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `structure` (bin "structure") due to 1 previous error
```

Rust 編譯器很貼心地告訴我們如何修正這個問題：

```shell
#[derive(Debug)]
19 |         struct Point {
```

依 Rust 編譯器建議，修正如下：

```rust {.line-numbers}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 0 };

println!("({}, {})", p.x, p.y);
println!("{:?}", p);
```

### println!("{:?}") and fmt::Debug

資料型別需要實作 `fmt::Debug` trait，才能在 `println!("{:?})` 列印。由於宣告的 `Point` 並沒有實作 `fmt::Debug` trait，因此編譯錯誤。

Rust 提供了 `#[derive(Debug)]` 來自動實作 `fmt::Debug` trait。`#[derive]` 是一個 macro，在編譯時期，會自動產生對應的程式碼，節省開發者的時間。使用 `#[derive(Debug)]` 前提是所有的 Field 型態，都需要實作 `fmt::Debug` trait。

```rust {.line-numbers}
struct Point(i32);

#[derive(Debug)]
struct Point3D(Point, Point, Point);

let p3d = Point3D(Point(0), Point(0), Point(0));
println!("{:?}", p3d);
```

由於 `Point` 並沒有實作 `fmt::Debug` trait，因此編譯錯誤如下：

```shell
Compiling structure v0.1.0 (/Users/kigi/Projects/kigi/learning_rust_from_go/ch07/structure)
error[E0277]: `Point` doesn't implement `Debug`
  --> src/main.rs:47:24
   |
46 |         #[derive(Debug)]
   |                  ----- in this derive macro expansion
47 |         struct Point3D(Point, Point, Point);
   |                        ^^^^^ `Point` cannot be formatted using `{:?}`
   |
   = help: the trait `Debug` is not implemented for `Point`
   = note: add `#[derive(Debug)]` to `Point` or manually `impl Debug for Point`
   = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider annotating `Point` with `#[derive(Debug)]`
   |
44 +         #[derive(Debug)]
45 |         struct Point(i32);
   |

For more information about this error, try `rustc --explain E0277`.
error: could not compile `structure` (bin "structure") due to 1 previous error
```

如先所述，編譯器提供了解決方法：

```shell
#[derive(Debug)]
45 |         struct Point(i32);
```

依照編譯器的建議，修正如下：

```rust {.line-numbers}
#[derive(Debug)]
struct Point(i32);

#[derive(Debug)]
struct Point3D(Point, Point, Point);

let p3d = Point3D(Point(0), Point(0), Point(0));
println!("{:?}", p3d);
```

## Method

Rust 使用 `impl` 來定義 Struct 的方法。在使用上，可以使用多個 `impl` 來定義不同的方法。

```rust {.line-numbers}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn distance(&self, p: &Point) -> f64 {
        let x = self.x - p.x;
        let y = self.y - p.y;
        ((x * x + y * y) as f64).sqrt()
    }
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

let p0 = Point::origin();
let p1 = Point::new(20, 30);
println!("{:?}", p0);
println!("{:?}", p1);

println!("distance from origin is {}", p1.distance(&p0));

let mut p2 = Point::new(10, 10);
p2.move_to(20, 5);
assert_eq!(p2.x, 20);
assert_eq!(p2.y, 5);
```

### Static Method

Rust 與 Scala 相同，可以定義 static method。在 Rust 中，static method 以 `::` 存取。Go 就沒有 static method。__`Self`__ 代表目前的型別。

```rust {.line-numbers}
impl Point {
  fn new(x: i32, y: i32) -> Self {
      Point { x, y }
  }

  fn origin() -> Point {
      Point { x: 0, y: 0 }
  }
}

let p0 = Point::origin();
let p1 = Point::new(20, 30);
```

### Immutable and Mutable Reference in Method

Rust 在 Method 使用上，與 Go 相同與 Scala 相同都是以 `.` 呼叫 Method。與一般 OOP 程式語言不同點是，Rust 沒有 __`this`__ 這個關鍵字，而是使用 __`&self`__ (immutable reference) 或 __`&mut self`__ (mutable reference) 來代表目前的實例。

```rust {.line-numbers}
impl Point {
  fn distance(&self, p: &Point) -> f64 {
      let x = self.x - p.x;
      let y = self.y - p.y;
      ((x * x + y * y) as f64).sqrt()
  }

  fn move_to(&mut self, x: i32, y: i32) {
      self.x = x;
      self.y = y;
  }
}
let p0 = Point::origin();
let p1 = Point::new(20, 30);

println!("{:?}", p0);
println!("{:?}", p1);

println!("distance from origin is {}", p1.distance(&p0));

let mut p2 = Point::new(10, 10);
p2.move_to(20, 5);
```

## More about Ownership

Stuct 的 Method 與 Function 一樣，都是遵循 Mutable 及 Ownership 規則。因此要修改 Struct 的欄位，必須使用 __`&mut self`__。

如果 Method 沒有使用 Reference，則會發生 Ownership 轉移，如下：

```rust {.line-numbers}
impl Point {
    fn add(self, (x, y): (i32, i32)) -> Self {
        Point {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

let p1 = Point::new(10, 10);
let p2 = p1.add((20, 5));
println!("{:?}", p2);
println!("{:?}", p1); // Error
```

1. `add` 沒有使用 Reference，因此會 Ownership 轉移。
1. `p1` 在 `add` 呼叫後，會 Ownership 轉移給 `add`，因此 `p1` 不能再使用。
1. 在 Rust 有些 Method 會消耗 (consume) 自己，尤其是 Operator Overloading；這類型 Method 會使用 `self` 而不是 `&self` 或 `&mut self`。在使用時，要特別注意。

## Cell and RefCell

一開始使用 Struct 與 Method 會卡關的情境是，不知道要如何安排 Method 的 `&self` `&mut self`。如果 Struct 大部分的欄位是 immutable，但有少數欄位需要 mutable 時，一開始都會將變數宣告成可變，如：`let mut p = Point::new(10, 10)`，這樣會造成整個 Struct 都是 mutable，也可以異動到其他唯讀的欄位。

Rust 提供了 `Cell` 與 `RefCell` 來解決這個問題，`Cell` 與 `RefCell` 除了可以用在 Struct Field 上，也可用在一般變數。`Cell` 與 `RefCell` 差別，在於 `RefCell` 提供修改內部資料的功能，適用在 Vector 這類的資料型別。

`Cell` 與 `RefCell` 基本觀念與差別如下：

- `Cell`: 修改變數指向記憶體，對應的效果如下：

  ```rust {.line-numbers}
  let mut x = 1;
  x = 2;
  ```

- `RefCell`: 沒有修改變數指向記憶體，但修改記憶體內容，對應的效果如下：

  ```rust {.line-numbers}
  let mut x = 1;
  let xx = &mut x;
  *xx = 2;
  ```

`Cell` 與 `RefCell` 範例如下：

```rust {.line-numbers}
use std::cell::{Cell, RefCell};

#[derive(Debug)]
struct Object {
    id: String,
    counter: Cell<u32>,
    flag1: Cell<Option<u32>>,
    flag2: RefCell<Option<u32>>,
    recorder: RefCell<Vec<u32>>,
}

impl Object {
    fn new(id: &str) -> Object {
        Object {
            id: id.to_string(),
            counter: Cell::new(0),
            flag1: Cell::new(None),
            flag2: RefCell::new(Some(1)),
            recorder: RefCell::new(Vec::new()),
        }
    }

    fn count(&self) {
        self.counter.set(self.counter.get() + 1);
    }

    fn double_count(&self) -> u32 {
        self.count();
        self.counter.replace(self.counter.get() + 1)
    }

    fn toggle_flag(&self) {
        self.count();
        let counter = self.counter.get();
        if counter % 2 == 0 {
            self.flag1.set(None);
            self.flag2.replace(Some(counter));
        } else {
            self.flag1.set(Some(counter));
            self.flag2.replace(None);
        }
        self.recorder.borrow_mut().push(counter);
    }
}

let obj = Object::new("obj");
println!("{:?}", obj);
obj.toggle_flag();
println!("{:?}", obj);
obj.toggle_flag();
println!("{:?}", obj);
println!("old counter: {}", obj.double_count());
println!("{:?}", obj);
obj.toggle_flag();
println!("{:?}", obj);
```

1. `use` 引用，類似 Go 的 `import`。如果引用的資料型別，在相同的套件 (crate) 下，可以使用 `{}` 引用，如:

    ```rust {.line-numbers}
    use std::cell::{Cell, RefCell};
    ```

    ，不用寫成多行，如:

    ```rust {.line-numbers}
    use std::cell::Cell;
    use std::cell::RefCell;
    ```

1. `Cell<T>` 與 `RefCell<T>` 是 Generic 宣告的語法。Generic 後面會再詳細說明。
1. `Cell` 提供 `get` 與 `set` 取得與修改 `Cell` 內的值。
1. `Cell` 提供 `replace` 修改 `Cell` 內的值，並回傳舊的值。
1. `Cell` 特別適用 Rust 數值型別，或有 `Copy` 特性的型別。
1. `RefCell` 提供 `borrow` 取得 immutable reference 與 `borrow_mut` 取得 mutable reference。
1. `RefCell` 的 `borrow` 與 `borrow_mut` 亦需遵循 Rust 的 Borrow 規則，否則會發生 __panic__。
1. `RefCell` 亦提供 `try_borrow` 與 `try_borrow_mut`，可以檢查是否允許借用，避免發生 __panic__。
1. `RefCell` 的 `borrow_mut` 可以取得 mutable reference，並修改內容。如：`self.recorder.borrow_mut().push(counter);`。
1. `RefCell` 也有 `replace` 方法，但是使用 `std::mem::replace` 置換記憶體內容。
1. `RefCell` 適用在 Vector 這類的資料型別，因為 `RefCell` 可以提供 mutable reference，並修改內容。但數值類型或有 `Copy` 特性的型別，也可使用。

PS. 我在初學 Rust 有個誤解，`Copy` 特性的資料型別要用 `Cell`，而沒有 `Copy` 特性的資料型別要用 `RefCell`。比較正確的理解，應該是，如果需要修改內部資料 (如 Struct 的  Field 或 Vector 內的值)，但不修改指向的記憶體位址時，需使用 `RefCell`。而需要修改變數指向記憶體時，需使用 `Cell`。
