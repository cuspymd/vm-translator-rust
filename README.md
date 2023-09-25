# VM Translator in Rust

![GitHub last commit](https://img.shields.io/github/last-commit/cuspymd/vm-translator-rust)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/cuspymd/vm-translator-rust)

This repository contains a Rust implementation of the VM Translator for the Hack computer, as described in Chapter 7 of the [Nand to Tetris](https://www.nand2tetris.org/course) course. With this VM Translator, you can translate VM code into Hack Assembly Language code.

## Prerequisites

- Rust is required to build and run this VM Translator. If you don't have Rust installed, you can get it from the [official Rust website](https://www.rust-lang.org/).

## Installation

To build and use this VM Translator, you can clone this repository and build it using Cargo:

```bash
git clone https://github.com/cuspymd/vm-translator-rust.git
cd vm-translator-rust
cargo build
```
## Usage

You can translate VM code to Hack Assembly Language code using the following command:

```bash
cargo run input.vm
```

This will generate an input.asm file in the same directory as your input VM code (input.vm).
