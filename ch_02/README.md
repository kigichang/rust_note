# Ch 02

- `cargo new` to generate project template.
- `println!` to print message in std out.
- `eprintln!` to print message in std err.
- `format!` to generate a String for specific format.
- `vec!` to generate vector.
  - `vec![v;n]`: v for value, n for size.
- binary in `target` folder
  - `debug`
  - `release`
- `fn` to declare function
- number type:
  - u64/i64
  - u8 for byte
  - f32/f64
  - isize/usize for platform dependency (32bit or 64bit)
- mut: declare variable is mutable, default is unmutable.
- suffix `!` means __marco__ NOT a function.
- `assert!` make sure argument must be true, or panic in debug and release mode.
- use `return` as early return. Rust uses last line without end of `;`  of block as return value.
  - it is like Scala. and end of `;` will return Unit (Void).

## test

- `#[test]` to write unit test function
- `cargo test` to run test.
- `assert_eq!` to test results.
- prefix `#` is a __attribute__ tag like Java annotation for compiler.

## import package

- `use` to import package.

## cargo

use `[workspace]` to put multiple projects into one folder. see __ch_02/Cargo.toml__

## reference

- `&`: borrow reference, it will not effect owner relation.
- `*`: de-reference.

`&String` auto-convert to `&str`.

## Result

- `Ok(v)` and `Err(e)`
- `expect`: return value or exit program.

## Closure

- `|args| {}`

## raw string

`r#""#`

## packages

- actix: web
- serde: serialize and de-serialize
- num: math complex
- image: write image

## Comment for Documation

- `///`

## Option

- `Some(T)`
- `None`

## for-loop

- `for i in 0..limit`: start with 0 and up to (but not including) limit.
- `for i in 0..=limit`: start with 0 and up to (and including) limit.

## Tuple

- `(T1, T2, ...)`
- `tuple.0` as first element, and so on.

## str

The `str` type, also called a __'string slice'__, is the most primitive string type. It is usually seen in its borrowed form, `&str`. It is alos the type of string literals, `&'static str`.

- `s[..index]`: start with 0 and up to (but not including) index
- `s[index+1..]`: start with index + 1, (index+1) must be <= length.

## Cast

- `as type`: `100 as f64`

## Debug

- `#[derive(Debug)]` for `{:?}` in `println!`.
