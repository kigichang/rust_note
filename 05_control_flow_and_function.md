# Flow Control and Function

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [Flow Control](#flow-control)
  - [if-else if-else](#if-else-if-else)
  - [if-let](#if-let)
  - [loop - break - continue](#loop---break---continue)
  - [loop - break 給值](#loop---break-給值)
  - [loop label](#loop-label)
  - [while](#while)
  - [for](#for)
  - [for - iter](#for---iter)
  - [for - enumerate](#for---enumerate)
- [Function](#function)

<!-- /code_chunk_output -->

## Flow Control

Rust 與 Go 在語法上類似，但使用上，Rust 的功能比 Go 強大許多，尤其是 `if-else` 上，Go 是 __Statement__ 而 Rust 是 __Expression__。__Expression__ 可以回傳值，而 __Statement__ 不行。__Expression__ 的設計，可以彌補在三元運算子上的不足，甚至比三元運算子更好用。

### if-else if-else

```rust {.line-numbers}
let a;
let b = -1;

if b <= 0 {
    a = "".to_string();
} else if b > 0 && b < 10 {
    a = format!("0{}", b);
} else {
    a = format!("{}", b);
}
assert_eq!(a, "");

// a = b > 0 ? b : 0;
let a = if b > 0 { b } else { 0 };
assert_eq!(a, 0);
```

### if-let

Rust 支援在 if 中，解構 Option / Result / Enum，達到簡化程式碼的目的。

```rust {.line-numbers}
let pt = Some((10, 20));

if let Some((x, y)) = pt {
    println!("x: {}, y: {}", x, y);
} else {
    println!("no point exists");
}
```

### loop - break - continue

loop 是無窮迴圈，可以使用 `break` 跳出迴圈，`continue` 跳過迴圈的一次迭代。

```rust {.line-numbers}
let mut counter = 0;

loop {
    counter += 1;
    if counter <= 5 {
        continue;
    }
    println!("{:?}", counter);
    if counter == 10 {
        break;
    }
}
println!("{:?}", counter);
```

### loop - break 給值

```rust {.line-numbers}
let mut counter = 0;
let a = loop {
    counter += 1;
    if counter == 10 {
        break counter / 2;
    }
};
println!("{}", a);
```

### loop label

```rust {.line-numbers}
let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
}
println!("End count = {count}");
```

### while

while 有條件的 loop，但 while 不支援用 `break` 給值。

```rust {.line-numbers}
let mut idx = 0;
let v = vec![1, 2, 3, 4, 5];
while idx < v.len() {
    idx += 1;
    if idx % 2 == 0 {
        continue;
    }
    println!("{}", v[idx]);
    if idx == 3 {
        break;
    }
}
```

### for

Rust 使用 range 的方式有兩種：

1. `..` 不包含結束值
2. `..=` 包含結束值
3. `...` 同 `..`，舊版的寫法，但容易與 `..` 混淆，已不再使用。

```rust {.line-numbers}
for i in 0..10 {
    println!("{}", i);
}

for i in 0..=10 {
    println!("{}", i);
}
```

### for - iter

透過  `iter` 來迭代 collection。

```rust {.line-numbers}
let v = vec!["a", "b", "c", "d", "e"];
for val in v {
    println!("{}", val);
}
```

### for - enumerate

透過 `enumerate` 取得 index 與 value。

```rust {.line-numbers}
let v = vec!["a", "b", "c", "d", "e"];
for (idx, val) in v.iter().enumerate() {
    println!("idx: {}, val: {}", idx, val);
}
```

## Function

Rust 在 function 沒有 variadic function 設計，可以用 slice 取代，或使用 macro 來達成。

```rust {.line-numbers}
fn my_print(args: &[&str]) {
    for arg in args {
        println!("{}", arg);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

- `->`: 來指定回傳值的型別。
- 可以不使用 `return` 以最後一行當作回傳值。

與 Scala 類似，原則上，傳入 function 的參數都是不可變的，如果要改變參數的值，必須使用 `mut` 關鍵字。

```rust {.line-numbers}
fn append(mut s: String) -> String {
    s.push_str(" world");
    s
}

let s = "hello".to_string();
let s = append(s);
println!("{}", s);

let mut s = "hello".to_string();
s = append(s);
println!("{}", s);
```

- `mut`: 來指定可變的變數。
