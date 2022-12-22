# Ch04 Ownership and Moves

Almost all major programming languages fall into one of two camps, depending on which of the two qualities they give up on:

- `Safety First`: uses Garbage Collection.
- `Control First`: leaves you in charge of freeing memory.

## Ownership

### Rust String and &str

![Rust String and &str](string.png)

### C String

![C String](cstring.png)

### Box and String

![Box and String](box_and_string.png)

### Vector and Struct

![Vector and Struct](vector_and_struct.png)

## Moves

### step 1 init

![move_step1](move_step1.png)

### step 2 let t = s;

![move_step2](move_step2.png)

### Move Occur

1. Returning Value from Function
1. Constructing new Values
1. Passing Values to a Function

## Copy Type

Types implementing `Copy` trait, like number types. Implementing `Copy` must implement `Clone` trait. __COPY__ type will clone value, and use cloned value to assign to, pass into or return from function.

## replace value in memory

- `std::mem::replace`
- declare Option in collection or struct and use `.take` to replace by `None`.

## Copy

- `#[derive(Copy, Clone)]`

## Reference Count

- `Rc` and `Arc`

> A value owned by an `Rc` pointer is immutable.
> Rust's memory and thread-safety guarantees depend on ensuring that no value is ever simultaneously shared and mutable.
