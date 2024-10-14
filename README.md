# binrubyscript

A Rust application template that embeds and runs Ruby code using [Artichoke](https://github.com/artichoke/artichoke). 

Based on the comments of @lopopolo at [artichoke/artichoke#367](https://github.com/artichoke/artichoke/issues/367#issuecomment-557806087).

Artichoke is an implementation of the Ruby language written in Rust.

## Features

- Embed Ruby scripts into a Rust binary.
- Execute Ruby code using the Artichoke interpreter.
- **Important**: The embedded Ruby scripts are included in their original, plain-text form within the binary. They are neither obfuscated nor compiled into bytecode.

### Prerequisites

- Rust toolchain (recommended using [rustup](https://rustup.rs/))

### Building

```sh
git clone https://github.com/abinoam/binrubyscript.git
cd binrubyscript
cargo build --release
cargo run
```

## Stay Updated with Artichoke's Single-Binary Efforts

For ongoing developments and discussions about embedding Ruby code into a single binary using Artichoke, you can follow the GitHub issues labeled [A-single-binary](https://github.com/artichoke/artichoke/labels/A-single-binary) in the Artichoke repository:

- **Artichoke GitHub Issues – A-single-binary**: [https://github.com/artichoke/artichoke/labels/A-single-binary](https://github.com/artichoke/artichoke/labels/A-single-binary)

These issues provide insights into the project's progress, challenges, and future plans related to creating single-binary Ruby applications.
