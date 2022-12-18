# Ch 02

- `cargo new` to generate project template.
- `println!` to print message
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
