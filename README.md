# Learning Rust from Go

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [前言](#前言)
- [開發環境](#開發環境)
- [主要學習資源](#主要學習資源)
- [Rust 基礎說明](#rust-基礎說明)
- [多執行緒](#多執行緒)
- [實作應用](#實作應用)
- [進階應用](#進階應用)
- [參考資料](#參考資料)
  - [crates](#crates)
  - [其他資源](#其他資源)

<!-- /code_chunk_output -->

## 前言

第一次使用 Rust 是在改寫區塊鏈 Side Project。主要是利用 [Hyperledger Sawtooth](https://www.hyperledger.org/use/sawtooth)，原先使用 Go，後來改用 Rust。發現 Rust 的速度有比較快，但不好學。這二、三年來斷斷續續重頭學習，也一直沒有好好整理。

最近又再重頭學習一次，並且把公司內原本 CGO 的函式庫，用 Rust 重做一次後，比較有心得。Rust 使用上與 Scala 有很多雷同的地方，目前筆者主要以 Go 開發，因此以 Go 與 Scala 的經驗，來記錄 Rust 的學習筆記。

## 開發環境

- Rust 版本: 1.76.0
- 開發環境: Mac OS (arm64)
- 開發工具: [VSCode](https://code.visualstudio.com/)
- 文件使用 [Markdown Preview Enhanced](https://github.com/shd101wyy/markdown-preview-enhanced) 撰寫，請安裝完環境後再閱讀。
- [Source on Github](https://github.com/kigichang/learning_rust_from_go)

## 主要學習資源

- [Rust 程式設計, 2/e (Programming Rust: Fast, Safe Systems Development, 2/e)](https://www.tenlong.com.tw/products/9786263242326)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)
- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)

## Rust 基礎說明

- [01 Introduction](01_introduction.md)
  1. 評估是否適合或需要學習 Rust
  1. 初步了解 Rust 運作方式
  1. 安裝開發環境
  1. 實作第一隻程式 - hello world

- [02 Basic Data Types](02_basic_data_types.md)
  1. Scalar Types
  1. Immutable and Mutable Variables
  1. Cast
  1. Overflow
  1. Scope, Shadowing and Freeze

- [03 Compound Data Types](03_compound_data_types.md)
  1. Tuple
  1. Array
  1. Vector
  
- [04 Slice and String](04_slice_and_string.md)
  1. Slice
  1. String and &str

- [05 Control Flow and Function](05_control_flow_and_function.md)
  1. return and early return
  1. if-elseif-else
  1. loop

- [06 Rust Memory Principles](06_rust_memory_principle.md)
  1. Ownership, Clone, Copy and Move
  1. References and Borrowing
  1. Lifetime

- [07 Struct and Method](07_struct_and_method.md)
  1. Struct
  1. Method
  1. Drive
  1. Ownership
  1. Some Mutable Field in Imutable Struct

- [08 Enum](08_enum.md)
  1. Enum
  1. Enum in Memory
  1. Define Method in Enum
  1. Option

- [09 Pattern Match](09_patch_match.md)
  - destructuring
  - match [https://doc.rust-lang.org/reference/patterns.html#identifier-patterns](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns)
  - if let

- [10 Traits & Generic](10_traits_and_generic.md)
  - traits
    - Default
    - Drop
    - Clone
    - Copy
  - generic
    - From / Into
    - Operator Overloading

- [11 Cargo and Crates](11_cargo_and_crates.md)
  1. crates
  1. cargo
  1. cross compile

- [12 Common Collections](12_common_collections.md)
  1. vector
  1. binary heap
  1. map
  1. set

- [13 Error Handling](13_error_handling.md)
  1. panic
  1. result
  1. catching errors and error propagation.

- [14 Testing](14_testing.md)
  1. unit test
  1. integration test
  1. test private function

- [15 Iteration and Clousure](15_iteration_and_clousure.md)
  1. iterator
  1. clousure

- [16 Pointers](16_pointers.md)
  1. Box and Dereferencing
  1. Rc and Arc
  1. Cell and RefCell

- [17 Macros](17_macros.md)

- [18 Unsafe](18_unsafe.md)

## 多執行緒

- [19 Concurrency](19_concurrency.md)
  
- [20 Async](20_async.md)

- Arc and Mutex
- Send and Sync
- tokio
  - async
  - thread
  - channel
    - select!
  - actor

## 實作應用

- Serde
  - json
  - toml
- clap
- log and log4rs
- actix
- diesel
- http client
- websocket client
- time package
  - std::time
  - chrono

## 進階應用

- [FFI and cbindgen](https://rust-lang.github.io/rust-bindgen/introduction.html)
- [PyO3](https://github.com/PyO3/pyo3)

## 參考資料

以下是我學 Rust 過程，記錄下來的一些資源，供大家參考。

### crates

- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
- [clap](https://github.com/clap-rs/clap)
- [serde-rs](https://github.com/serde-rs/)
- [tokio.rs](https://tokio.rs/tokio/tutorial)
- [actix.rs](https://actix.rs/)
- [redis-rs](https://github.com/redis-rs/redis-rs)
- [tonic](https://github.com/hyperium/tonic)
- [gRPC-rs](https://github.com/tikv/grpc-rs)

### 其他資源

- [Comprehensive Rust by Google](https://google.github.io/comprehensive-rust/)
- [Rust and WebAssembly](https://rustwasm.github.io/docs/book/introduction.html)
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
- [The bindgen User Guide](https://rust-lang.github.io/rust-bindgen/introduction.html)
- [The Rust Reference](https://doc.rust-lang.org/reference/introduction.html)
- [Rust语言圣经(Rust Course)](https://course.rs/about-book.html)
- [PyO3 user guide](https://pyo3.rs/)
- [The \`wasm-bindgen\` Guide](https://rustwasm.github.io/wasm-bindgen/introduction.html)
- [The Rustonomicon](https://doc.rust-lang.org/beta/nomicon/leaking.html)
- [CXX — safe interop between Rust and C++](https://cxx.rs/index.html)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Rust语言中文社区](https://rustcc.cn/)
- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
- [How to bind (Python + NumPy) with (Rust + Ndarray)](https://itnext.io/how-to-bind-python-numpy-with-rust-ndarray-2efa5717ed21)
