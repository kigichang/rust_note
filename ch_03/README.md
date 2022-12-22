# Ch03 Fundamental Types

- __char__ is __32__ bit unicode character.
- tuple: mix types `(T1, T2, ...)`
  - unit `()`: empty tuple.
- struct:
  - Named-filed struct:

    ```rust
    struct S {
        x: f32,
        y: f32
    };
    ```

  - Tuple-like struct:

    ```rust
    struct T (i32, char);
    ```

  - Unit struct, has __No__ fields

    ```rust
    struct U;
    ```

- Enumeration:

    ```rust
    enum Attend {
        OnTime,
        Late(u32),
    }
    ```

- Box: owning pointer to value in Heap.

    ```rust
    let attend = Box::new(Attend::Late(10))
    ```

- String: __UTF8__ string, __Dynamically__ sized.
  String represents its text as a sequence of UTF8 bytes, not as an array of characters.
  Rust character represents a single Unicode character, as a 32-bit value.
  
  ```rust
    "ラーメン: ramen".to_string()
  ```

- str is string slice as primitive string.
  - `&str` reference to str, __NON-OWNING__ pointer to UTF8 text.
  - index in byte

    ```rust
    let s = "そば: soba";
    println!("{}", s);
    println!("{}", &s[0..3]);
    println!("{}", &s[3..6]);
    println!("{}", &s[6..7]);
    ```

- array: `[f64; 4]`, `[u8; 256]`, fixed size, and elements all of same type.

  ```rust
  let a1 = [1, 2, 3, 4];
  println!("a1 = {:?}", a1);
  ```

- Result:
  - Ok(V)
  - Err(e)

- Option:
  - Some(T)
  - None

- `&dyn Any`, `&mut dny Read`: Trait Object, reference to any value that implements a given set of methods.

- `fn(&str) -> bool`: pointer to function.
- `|a, b| { a*a + b*b }`: clousure.

## assignment

- retreive value from block

  ```rust
  let x = {
          let y = 1;
          y + 1
  };
  ```

- `&[u8]`, `&mut [u8]`: reference to slice, reference to a portion of an array or vector, comprising __Pointer__ and __Length__.

## marco

- `todo!()`: allow compilation and not impement.

## Checked, Wrapping, Saturating, and Overflowing Arithmetic

- checked_xxx: return Option
- wrapping_xxx: return the value equivalent to the mathematically correct result modulo range of the value.
- saturating_xxx: return the representable value that is __ClOSEST__ to the mathematically correct result. In other words, the result is "clamped" to the maximum and minimum values the type can represent.
- overflowing_xxx: rethrn a tuple `(result, bool)`, where result is what wrapping version of the function would return, and overflowed is a bool indicating whether an overflow occurred.

## Reference

It's easiest to get started by thinking of references as Rust's basic pointer type. At run time, a reference to an i32 is a __SINGLE MACHINE WORD__ holding the address of i32, which may be on the stack or in the heap.

`&x` produces a reference to x; in Rust terminology, we say that it borrows a refernece to x; Given a reference `r`, the expression `*r` referes to the value r points to (de-ref).

`&T`: an immutalbe, shared reference. You can have __MANY__ shared references to a given value at a time, but they are read-only. like `const T*` in C.

`&mut T`: a mutable, EXCLUSIVE refernece. You can read and modify the value it points to, as with a `T*` in C. But for as long as the reference exists, you may not have any other referernce of any kind to that value.

> Rust use this dichotomy between shared and mutable references to enforce a __SINGLE WRITER or MULTIPLE READERS__ rule: either you can read and write the value, or it can be shared by any number of readeres, __BUT NEVER BOTH AT THE SAME TIME__.

## Arrays, Vectors, and Slices

The `sort` method is actually defined on __SLICES__, but since it takes its operand by referernce. Rust __IMPLICITLY__ produces a `&mut [i32]` slice refering to the entire array and passes that to `sort` to operate on. In fact, the `len` method is a slice method as well.

Length and Capacity in Vector.

![slice](slice.png)

## Strings

`.to_string()` and `format!()` to generate String.

![string and &str](string.png)

| | Vec\<T\> | String |
| - | - | - |
| Automatically frees buffers | Yes | Yes |
| Growable | Yes | Yes |
| `::new()` and `::with_capacity()` type-associalted functions | Yes | Yes |
| `.reserve()` and `.capacity()` methods | Yes | Yes |
| `.push()` and `.pop()` methods | Yes | Yes |
| Range syntax `v[start..stop]` | Yes, returns &[T] | Yes, returns &str |
| Automatic conversion | &Vec\<T\> to &[T] | &String to &str |
| Inherits methods | From &[T] | From &str |

## Other String

__Stick to String and &str for UNICODE text__.

- File and Path: `std::path::PathBuf` and `&Path`
- binary data not UTF8 encoded at all: `Vec<u8>` and `&[u8]`
- environment variable names and command-line arguments in the NATIVE form presented by the OS: `OsString` and `&OsStr`
- C FFI: `std::ffi::CString` and `&CStr`
