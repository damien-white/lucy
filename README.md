<div align="center">
    <hr /><br/>
    <img src="assets/logo.svg" width="400" alt="Lucy logo" />
    <hr />
    <h2 align="center">  
        Parse, inspect and transform arbitrary JSON data
    </h2>

[![Crates.io](https://img.shields.io/crates/v/lucy.svg)](https://crates.io/crates/lucy)
[![Docs.rs](https://docs.rs/lucy/badge.svg)](https://docs.rs/lucy)
[![CI](https://github.com/dark-fusion/lucy/workflows/CI/badge.svg)](https://github.com/dark-fusion/lucy/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/lucy/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/lucy?branch=main)

</div>

## Description

Lucy parses JSON data from sources that are represented as raw bytes. Examples of this include, but
are not limited to files and sockets.

The definition of "valid JSON syntax" is taken directly from the
official [IETF RFC](https://datatracker.ietf.org/doc/html/rfc8259).

### Project Goals

- Support all JSON data types. These include:
    - Primitives:
        - `string`
        - `number`
        - `boolean`
        - `null`
    - Structured:
        - `object`
        - `array`
- Parse raw bytes into meaningful, structured data
- Use zero-copy operations whenever possible
- Avoid excessive memory allocations
- Provide utilities and helpers for integrating Lucy into your projects

### Project Non-goals:

- Compatibility with `Serde`
- Support for non-JSON formats

### Technologies

Lucy is built with:

- [Rust programming language](https://rust-lang.org/)
- [nom](https://github.com/Geal/nom)
- [bytes](https://github.com/tokio-rs/bytes)
- TBA

## Installation

### Prerequisites

1. You must have cargo installed and an up-to-date version of Rust.
    - Please visit the official website
      for [installation instructions](https://www.rust-lang.org/tools/install).
2. Ensure the project compiles. From the project root directory, run:
    - `cargo build` or `just compile`

## License

Licensed under the [MIT License](LICENSE):

- You may also find a copy at http://opensource.org/licenses/MIT

## Contribution

Contributions intentionally submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
