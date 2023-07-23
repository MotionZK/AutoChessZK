# Zero Knowledge Auto Chess

Welcome! This repo is a small showcase of a multi-guest proof system that resembles Motion's module system. The code is incomplete at the moment, but if you have experience with building on RISC Zero or other zkVMs feel free to take a look around. For everyone else, take a look at [Introduction to Motion Modules](https://motionzk.gitbook.io/docs/motion-modules/introduction-to-motion-modules) for an deep dive into what we're building.

## Getting Started

First, make sure [rustup](https://rustup.rs) is installed. This project uses a [nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) version of [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html). The [`rust-toolchain`](rust-toolchain) file will be used by `cargo` to automatically install the correct version.

To build all methods and execute the method within the zkVM, run the following command:

```
cargo run
```
