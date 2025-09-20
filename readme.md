# Tiralabra

TiraCompress is a file compressor.

[➜ Spec / määrittelydokumentti](doc/specification.md) [➜ Testing doc / testausdokumentti](doc/testing.md) 

[➜ report 1](doc/week1.md)
[➜ report 2](doc/week2.md)
[➜ report 3](doc/week3.md)
[➜ report 4](doc/week4.md)


## Development

**Setup**
- Clone the repo
- Install [Rust](https://www.rust-lang.org/) if you don't have it already. Linux users may also may find it in the native package manager.

**Building**
  - Run `cargo build` at repository root. [read more](https://doc.rust-lang.org/cargo/commands/cargo-build.html)
    - Use `--release` flag to make an optimized build, or die of old age waiting for large files to compress.  
  - Get your executable from `target/<yourtarget>/`
  
**Other**
  - Just build & run: `cargo run --release`
  - Tests: `cargo test`
  - Test coverage: `cargo llvm-cov --html`
    - Report can be found at `target/llvm-cov/html/index.html`
    - Install the coverage tool by running `cargo install cargo-llvm-cov`
  - Linter: `cargo clippy`
