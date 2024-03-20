# Rust 簡介

<!-- @import "[TOC]" {cmd="toc" depthFrom=2 depthTo=3 orderedList=false} -->

<!-- code_chunk_output -->

- [本章節重點](#本章節重點)
- [前言](#前言)
- [學習 Rust 的建議](#學習-rust-的建議)
- [Rust 特性簡介](#rust-特性簡介)
- [Rust Principle](#rust-principle)
- [Rust 開發環境](#rust-開發環境)
  - [編譯器安裝](#編譯器安裝)
  - [IDE Tools: VSCode and Zed](#ide-tools-vscode-and-zed)
  - [Cargo 簡介](#cargo-簡介)
- [參考資料](#參考資料)

<!-- /code_chunk_output -->

## 本章節重點

1. 評估是否適合或需要學習 Rust
1. 初步了解 Rust 運作方式
1. 安裝開發環境
1. 實作第一隻程式 - hello world

## 前言

以筆者實戰過的程式語言，如 C/C++, Java, C#, Scala, Go 中，Rust 相對不好學習。主因是必須牽就於 Rust 對記憶體管理的要求，才能編譯成功。依筆者經驗，初期必須花較多的時間在了解 Rust 記憶體管理上的思維，這對不熟悉程式語言運作原理的初學者來說，會是一道很高的門檻。

為什麼筆者會想學 Rust。以下是個人想法，如果你也有相同的想法或需求，那麼你也可以考慮學習 Rust；如果沒有，建議用原本的解決方案即可。如果你是初學第一個程式語言，**非常不建議**學習 Rust。

筆者想學習 Rust 的原因：

1. 筆者脫節 C/C++ 太久，後來 C++ 新加很多功能；筆者覺得與其重新學習 C++，不如學習一門新的程式語言。
1. 筆者近期以 Go 為主，有整合 C/C++ 的需求，有時候會覺得 CGo 並不是那麼好用。
1. Go 的自由度很高，但也因此常誤用記憶體，此項純屬個人問題。筆者想如果可以在編譯時期，就可以解決這些問題，那產品品質相對會比較高。
1. 就目前使用 Rust 經驗，如果開發期已經通過編譯，在測試期，基本上已經沒什麼問題，可以省下後期的維護。
1. 支援 WebAssembly。
1. **學完 Go 想找下一個程式語言學習。**

摘錄自 [xAI](https://x.ai/) 官網：

>Rust has proven to be an ideal choice for building scalable, reliable, and maintainable infrastructure. It offers high performance, a rich ecosystem, and prevents the majority of bugs one would typically find in a distributed system. Given our small team size, infrastructure reliability is crucial, otherwise, maintenance starves innovation. Rust provides us with confidence that any code modification or refactor is likely to produce working programs that will run for months with minimal supervision.
>Rust 已被證明是建造可擴展、可靠且可維護的基礎設施的理想選擇。它提供高性能、豐富的生態系統，並防止分散式系統中通常會發現的大多數錯誤。鑑於我們的團隊規模較小，基礎設施的可靠性至關重要，否則維護將缺乏創新。Rust 讓我們充滿信心，任何程式碼修改或重構都可能產生可以在最少監督的情況下運行數月的工作程序。 (翻譯自 微軟)

## 學習 Rust 的建議

以下是我對學習 Rust 的建議，如果你也想學習 Rust，可以參考看看。

1. 如果只熟悉 **弱型別** 的程式語言，如 PHP, Javascript 的話，建議不要學習 Rust。因為既然使用弱型別的解決方案，代表專案的型態或團隊建置，已符合需求。Rust 無法替代原先的解決方案，學習 Rust 只會增加開發成本。
1. 如果已熟悉一種 **強型別** 的程式語言，務必先了解其在記憶體的運作與管理方式，在學習 Rust 時，可以參照比對，會比較容易理解為什麼 Rust 會這樣設計。
1. 在學習 Rust 過程，不要學完基本語法後，就想實作資料結構或演算法問題，會很容易失敗而在這階段放棄。筆者自己就經歷過此過程。
1. Rust 不像 Go 已經內建很好用的功能，因此在實戰上，需要依賴很多第三方的套件。而這些第三方套件有些很成熟，有些剛開發，但也有很久沒維護，在實戰上要慎選。

## Rust 特性簡介

Rust 基本的語言特性：

1. 預設 UTF-8 編碼
1. 強型別
1. 沒有 Garbage Collection (GC)；雖然可以自行釋放記憶體，但基本上還是由 Rust 編譯器管理生命週期會比較好。
1. 沒有 Nil (Null)
1. 沒有繼承，所以也就沒有 OOP。
1. 有 Macro, Generic, Enum, and Trait (Go Interface)。
1. 有 Thread, Channel 功能。
1. 有 Async。
1. 有 Closure。

以下是以筆者經驗的比較。筆者 Java 經驗停在 JDK 8 前的版本，之後轉用 Scala，Scala 之後就以 Go 為主。Scala 版本，筆者是停留在 2.11 版本。如果對 Scala 的理解有錯，還請指正。

| 比較 | Rust | Go | Scala |
| - | - | - | - |
| Artifact | Machine Code | Machine Code with **Go Runtime**，也因此檔案會比較大 | JVM Bytecode |
| 跨平台 | Y | Y | Y (依賴 JVM) |
| :+1: **Garbage Collection (GC)** | **N** | Y | Y |
| Object-Oriented Programming| N (沒有繼承) | N (沒有繼承) | Y |
| :+1: **Functional Programming (FP)** | Y | Y (支援程度不如 Rust / Scala) | Y |
| :+1: **Generic (泛型)** | Y | Y (Go 1.18 之後版本，目前還在進步中，支援功能，遠遠不及 Rust / Scala) | Y |
| Unsigned 型別 | Y | Y | N |
| Unit 型別 | **Y** `()` | N | Y `Unit` |
| :+1: **NULL** | N, use `Option` instead  | Yes, `nil` | N, use `Option` instead |
| :+1: **Tuple** | **Y** | Y (只支援在 **return** 時回傳多組值) | Y |
| :+1: **Interface** | Y (Trait) | Y (Interface) | Y (Trait) |
| :+1: **Default Function in Interface** | **Y** | N (不能在 interface 實作 function) | Y |
| :+1: **Enum** | **Y** | N (只能用 const + iota 模擬) | Y |
| :+1: **Macro** | **Y** | N | Y |
| Inline | **Y** | N (Go compiler 會自動判斷是否要 inline [^go_inline]) | Y |
| :+1: **Pattern Matching** | **Y** | N | Y |
| :+1: **if-then-else** | **expression (可 return 值)** | statement | expression (可 return 值) |
| :+1: **Operator Overloading** | **Y** | N | Y |
| Concurrency / Parallel / Async | thread / async / channel / actor | goroutine / channel / wait group | thread / async / actor |
| :+1: **Error Handling** | Result / Option | Error | Try / Either / Option |
| Syntactic Sugar (語法糖) | 中 | 弱 | 太強 (會像在寫天書) |
| :+1: **Package 管理** | 較複雜，但也更有彈性 | 較簡單 | 較簡單 |
| Coding Style | camel (Struct & Trait) and snake (Varible & Function) | camel | camel |

[^go_inline]: [Compiler And Runtime Optimizations](https://github.com/golang/go/wiki/CompilerOptimizations#function-inlining)

## Rust Principle

既然 Rust 是由編譯器管理記憶體的生命週期，依筆者經驗，有幾個觀念需要先建立：

1. 編譯器會自動加入釋放無用的記憶體程式碼，也就是說，當編譯器無法偵測某記憶體生命週期時，編譯器會拒絕編譯。
1. 基於存入 Stack 記憶體的資料，其容量大小 (Size) 必須固定，因此在編譯時期就須已知，否則編譯器會拒絕編譯。
1. 為避免同一區塊記憶體被重覆釋放，一個記憶體區塊只能有一個擁有者(Ownership 設計)，也就是說當有多個擁有者時，編譯器會拒絕編譯。
1. 承上，Rust 允許記憶體的擁有權轉移，當變數喪失記憶體擁有權後，該變數不能再使用，否則編譯器會拒絕編譯。

## Rust 開發環境

### 編譯器安裝

依[官網說明](https://www.rust-lang.org/zh-TW/tools/install)安裝，Rust 支援 Windows、Linux、MacOS，筆者是在 Apple Macbook Pro M2 上安裝。

### IDE Tools: VSCode and Zed

目前建議使用 [VSCode](https://code.visualstudio.com/)，另一個選擇是 [Zed](https://zed.dev/)。兩者底層都是使用 [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)。Zed 會自動偵測到編輯 rust 檔案時，會自動安裝 rust-analyzer。

### Cargo 簡介

Cargo 是 Rust 的專案管理工具，類似 Java 的 Maven (`mvn`), Scala 的 `sbt` 或 Go 的 `go`。Cargo 會自動建立專案的目錄結構，並且自動產生專案的基本設定檔案。

1. 開發目錄下執行 `cargo new hello-world`，會產生 **hello-world** 目錄如下：

    ```shell
    hello-world
    ├── Cargo.toml
    └── src
        └── main.rs
    ```

    **Cargo.toml**: 是專案檔，如同 Go 的 go.mod，Java 的 pom.xml 或 Scala 的 build.sbt。Cargo.toml 功能很強大，可以設定專案的名稱、版本、作者、依賴套件、編譯器版本等等。
    **src**: 是放 source code。

1. 進入 `hello-world` 目錄(`cd hello-world`)，執行 `cargo run`，會自動編譯並執行程式。

    ```shell
    $ cargo run
        Compiling hello-world v0.1.0 (/Users/kigi/Projects/kigi/learning_rust_from_go/hello-world)
          Finished dev [unoptimized + debuginfo] target(s) in 0.56s
            Running `target/debug/hello-world`
    Hello, world!
    ```

1. 執行成功後，會 **hello-world** 目錄下，產生 **target** 目錄，rust 會將編譯成功的執行檔放在這，類似 Visual Studio 的目錄架構。rust 編譯器預設是編譯 **Debug** 版本。因此會在 **target/debug** 發現已編譯完成的執行檔 **hello-world**。

1. 測試執行

    ```shell
    $ ./target/debug/hello-world
    Hello, world!
    ```

1. 編譯 Release 版本：`cargo build --release`，編譯完成後，會在 **target/release** 目錄下發現已編譯完成的執行檔 **hello-world**。

    ```shell
    $ ./target/release/hello-world
    Hello, world!
    ```

#### Hello World 程式碼

```rust {.line-numbers}
fn main() {
    println!("Hello, world!");
}
```

1. 與大部分程式語言一樣，程式由 `main()` 開始執行。
1. 利用 `println!("Hello, world!")` 輸出 **Hello, world!**。類似 Go 的 `fmt.Println("Hello, world!")`。
1. 在 Rust 中。函式名稱有 _`!`_ 代表是 Macro，而不是一般的函式。
1. Rust 格式化輸出．請見 [https://doc.rust-lang.org/std/fmt/](https://doc.rust-lang.org/std/fmt/)。

## 參考資料

1. [Rust fact vs. fiction: 5 Insights from Google's Rust journey in 2022](https://opensource.googleblog.com/2023/06/rust-fact-vs-fiction-5-insights-from-googles-rust-journey-2022.html)
1. [Comprehensive Rust (by Google)](https://google.github.io/comprehensive-rust/)
1. [First Rust Code Shows Up in the Windows 11 Kernel](https://www.thurrott.com/windows/windows-11/282995/first-rust-code-shows-up-in-the-windows-11-kernel)
1. [Rust for Windows](https://github.com/microsoft/windows-rs)
