# Collections

- Vector
- Heap
- Map
- Set

## Vector

Rust 的 Vector 是一種複合型別，可以包含相同型別的元素，並且 Vector 的長度是可變的。Vector 比較操作上，比較像是 Go 的 Slice。

```rust {.line-numbers}
fn main() {
    let mut vec = vec![1, 2, 3, 4, 5]; // 宣告 vector
    vec.push(6); // add element
    vec.push(7); // add element
    println!("{:?}", vec);
    assert_eq!(vec.get(vec.len() - 1), Some(&7_i32)); // get element

    for i in 0..vec.len() {
        print!("{} ", vec[i]);
    }
    println!();

    let mut vec = vec![0_i32; 10]; // 宣告 vector，給予初始值與長度
    for i in 0..vec.len() {
        vec[i] = i as i32;
    }
    println!("{:?}", vec);

    let slice = &vec[2..5]; // get slice of vector
    println!("{:?}", slice);

    for i in 0..slice.len() {
        print!("{} ", slice[i]);
    }
    println!();
}
```
