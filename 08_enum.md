# Enum

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [前言](#前言)
- [Declare Enum](#declare-enum)
- [Enum with Different Types](#enum-with-different-types)
- [Enum in Memory](#enum-in-memory)
  - [Assign Enum Type and Value](#assign-enum-type-and-value)
- [Enum and Method](#enum-and-method)
- [Enum and Option](#enum-and-option)

<!-- /code_chunk_output -->

## 前言

Enum 在軟體設計上，適用於有關系統狀態值上，透過 Enum 可以更清楚的表達系統的狀態，也可以避免使用者輸入錯誤的值。

Rust 的 Enum 與 Scala 相同，可以定義一種型別，包含一種或多種資料類型。Go 在 Enum 功能就非常缺乏。

## Declare Enum

最簡單的 Enum 用法如下：

```rust {.line-numbers}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn show(d: Direction) {
    println!("{:?}", d);
}

fn show2(d: &Direction) {
    println!("{:?}", d);
}

let up = Direction::Up;
let down = Direction::Down;
let left = Direction::Left;
let right = Direction::Right;

show(up);
show2(&down);
show(left);
show2(&right);

println!("{:?}", down);
println!("{:?}", up); // Error
```

1. 使用 `enum` 關鍵字宣告 Enum。
1. 利用 `::` 使用 Enum 的值。
1. Enum 與 Struct 也可以使用 `#[derive()]` 讓編譯器自動產生對應的程式碼。如: `#[derive(Debug)]`。
1. Enum 亦遵循 Reference / Ownership 的規則。

## Enum with Different Types

Enum 的值可以是不同的型別，如下：

```rust {.line-numbers}
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Polygon {
    Triangle(Point, Point, Point),
    Rectangle(Point, Point, Point, Point),
}

let p1 = Polygon::Triangle(
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
);

let p2 = Polygon::Rectangle(
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 0, y: 1 },
);

println!("{:?}", p1);
println!("{:?}", p2);
```

## Enum in Memory

Rust Enum 配置記憶體的方式，使用 [Tagged Union](https://en.wikipedia.org/wiki/Tagged_union)，除了配置最大使用量外，還有會額外的配置一個 Tag 來記錄目前使用的值。

```rust {.line-numbers}
struct Empty;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Triangle(Point, Point, Point);

#[derive(Debug)]
struct Rectangle(Point, Point, Point, Point);

#[derive(Debug)]
enum Polygon {
    Triangle(Point, Point, Point),
    Rectangle(Point, Point, Point, Point),
}

let p1 = Polygon::Triangle(
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },
);

let p2 = Polygon::Rectangle(
    Point { x: 0, y: 0 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 0, y: 1 },
);

println!("size of Empty: {}", std::mem::size_of::<Empty>()); // 0
println!("size of Point: {}", std::mem::size_of::<Point>()); // 8
println!("size of Triangle: {}", std::mem::size_of::<Triangle>()); // 24
println!("size of Rectangle: {}", std::mem::size_of::<Rectangle>()); // 32
println!("size of Polygon: {}", std::mem::size_of::<Polygon>()); // 36
println!("size of Polygon::Triangle: {}", std::mem::size_of_val(&p1)); // 36
println!("size of Polygon::Rectangle: {}", std::mem::size_of_val(&p2)); // 36
```

1. 使用 `std::mem::size_of::<T>` 取得型別 T 的記憶體大小。類似 C 的 `sizeof()`。
1. `Empty` 是空的 Struct, 使用 __0__ byte。
1. `Point` 的大小是 8 bytes，因為 `i32` 是 4 bytes，有兩個 `i32` 組成。
1. `Triangle` 的大小是 24 bytes，因為有三個 `Point`。
1. `Rectangle` 的大小是 32 bytes，因為有四個 `Point`。
1. `Polygon` 的大小是 36 bytes，除了 `Rectangle` 的 32 bytes 外，還有 4 bytes 的 Tag。

### Assign Enum Type and Value

如果 Enum 不指定型別時，會預設是 u8；也可以使用 `#[repr()]` 來指定型別，如: `#[repr(i32)]`。

如果 Enum 不指定值時，會從 __0__ 開始遞增。也可以使用 `=` 指定值。

以上方式，很適用於系統狀態值的設計。

```rust {.line-numbers}
enum Corner {
    LeftTop,
    LeftBottom,
    RightTop,
    RightBottom,
}

println!("size of Corner: {}", std::mem::size_of::<Corner>()); // 1

unsafe {
    println!(
        "Corner::LeftTop is {}",
        std::mem::transmute::<Corner, u8>(Corner::LeftTop)
    ); // 0

    println!(
        "Corner::LeftBottom is {}",
        std::mem::transmute::<Corner, u8>(Corner::LeftBottom)
    ); // 1

    println!(
        "Corner::RightTop is {}",
        std::mem::transmute::<Corner, u8>(Corner::RightTop)
    ); // 2

    println!(
        "Corner::RightBottom is {}",
        std::mem::transmute::<Corner, u8>(Corner::RightBottom)
    ); // 3
}

#[derive(Debug)]
#[repr(i32)]
enum Direction {
    Up = 3,
    Down,
    Left,
    Right,
}

println!("size of Direction: {}", std::mem::size_of::<Direction>()); // 4
unsafe {
    println!(
        "Direction::Up is {}",
        std::mem::transmute::<Direction, i32>(Direction::Up)
    ); // 3

    println!(
        "Direction::Down is {}",
        std::mem::transmute::<Direction, i32>(Direction::Down)
    ); // 4

    println!(
        "Direction::Left is {}",
        std::mem::transmute::<Direction, i32>(Direction::Left)
    ); // 5

    println!(
        "Direction::Right is {}",
        std::mem::transmute::<Direction, i32>(Direction::Right)
    ); // 6
}
```

1. 如 `Corner`，不指定資料型別時，記憶體使用 1 byte。
1. `Corner` 的值是從 0 開始遞增。
1. `Direction` 使用 `#[repr(i32)]` 指定型別是 __i32__，記憶體使用 4 bytes。
1. `Direction` 中的第一個值 `Up` 指定為 __3__，因此從 __3__ 開始遞增。
1. Rust Enum 賦值的行為，與 Go 的 `iota` 類似。
1. `unsafe` 同 Go 的用法，Scope 內的程式碼，在編譯時不會做有關安全性的檢查，等同需要自行保證安全性。
1. `std::mem::transmute` 是將型別轉換成另一種型別的函式，是 unsafe 操作。

## Enum and Method

Enum 也可以像 Struct 一樣，定義 Method。以下的例子，是使用 Rust 實作 [Scala Either](https://www.scala-lang.org/api/2.13.6/scala/util/Either.html) 的功能。

```rust {.line-numbers}
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    fn is_right(&self) -> bool {
        match self {
            Either::Right(_) => true,
            _ => false,
        }
    }

    fn is_left(&self) -> bool {
        !self.is_right()
    }

    fn get_right(&self) -> Option<&R> {
        match self {
            Either::Right(r) => Some(r),
            _ => None,
        }
    }

    fn get_left(&self) -> Option<&L> {
        match self {
            Either::Left(l) => Some(l),
            _ => None,
        }
    }

    fn to_option(self) -> Option<R> {
        match self {
            Either::Right(r) => Some(r),
            _ => None,
        }
    }
}

let e1: Either<i32, ()> = Either::Left(1);
let e2: Either<(), i32> = Either::Right(2);

println!(
    "{:?}, {}, {}, {:?}, {:?}",
    e1,
    e1.is_left(),
    e1.is_right(),
    e1.get_left(),
    e1.get_right(),
);
println!("{:?}", e1.to_option());

println!(
    "{:?}, {}, {}, {:?}, {:?}",
    e2,
    e2.is_left(),
    e2.is_right(),
    e2.get_left(),
    e2.get_right(),
);
println!("{:?}", e2.to_option());
```

1. `enum Either<L, R>` 結合 Generic 的功能，可以定義任何 `L` 和 `R` 的型別。
1. `impl<L, R> Either<L, R>` 定義 `Either` 的 Method。
1. Generic 的使用方式，後面會再詳細說明。
1. `match` 是 Rust 的 Pattern Match，後面會再詳細說明。

## Enum and Option

Rust 的 Option 本身就是一個 Enum；結構上，與上面的 `Either` 非常類似，如下：

```rust {.line-numbers}
#[derive(Copy, PartialOrd, Eq, Ord, Debug, Hash)]
#[rustc_diagnostic_item = "Option"]
#[lang = "Option"]
#[stable(feature = "rust1", since = "1.0.0")]
pub enum Option<T> {
    /// No value.
    #[lang = "None"]
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value of type `T`.
    #[lang = "Some"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}

impl<T> Option<T> {
    ...
}
```
