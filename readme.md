# Tiralabra

TiraCompress is a file compressor.

[➜ Spec / määrittelydokumentti](doc/specification.md)

[➜ report 1](doc/week1.md)
[➜ report 2](doc/week2.md)


## Development

**Setup**
- Clone the repo
- Install [Rust](https://www.rust-lang.org/) if you don't have it already. Linux users may also may find it in the native package manager.

**Building**
  - Run `cargo build` at repository root. [read more](https://doc.rust-lang.org/cargo/commands/cargo-build.html)
  - Get your executable from `target/<yourtarget>/`
  
**Other**
  - Just build & run: `cargo run --release`
  - Tests: `cargo test`
  - Test coverage: `cargo llvm-cov --html`
    - Report can be found at `target/llvm-cov/html/index.html`
    - Install the coverage tool by running `cargo install cargo-llvm-cov`
  - Linter: `cargo clippy`