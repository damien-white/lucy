<div align="center">
    <hr /><br/>
    <img src="assets/logo.svg" width="400" alt="Lucy logo" />
    <hr />
    <h2 align="center">  
        JSON validation and processing service
    </h2>

[![Crates.io](https://img.shields.io/crates/v/lucy.svg)](https://crates.io/crates/lucy)
[![Docs.rs](https://docs.rs/lucy/badge.svg)](https://docs.rs/lucy)
[![CI](https://github.com/dark-fusion/lucy/workflows/CI/badge.svg)](https://github.com/dark-fusion/lucy/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/lucy/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/lucy?branch=main)

</div>

## Description

Lucy is a fast, efficient JSON validation and processing service.

Lucy parses JSON from sources that produce raw bytes, such as network sockets. This allows Lucy to
easily integrate with new and existing systems.

### JSON Syntax

The specification that defines valid JSON syntax and the format itself can be found
within [IETF RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259).

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
    - These optimizations will be "best-effort" until the release of an MVP
- Avoid excessive memory allocations
- Provide utilities and helpers for integrating Lucy into your projects

### Technologies

Lucy is written in [Rust](https://rust-lang.org/) for its ability to provide high-speed runtime
performance, resource efficiency and various memory-safety guarantees.

Rust's vibrant ecosystem contains support for asynchronous IO and parallel processing (concurrency
and parallelism).

#### External Dependencies

Lucy depends on a few well-maintained crates:

- [nom](https://github.com/Geal/nom)

## License

Licensed under the [MIT License](LICENSE):

- You may also find a copy at http://opensource.org/licenses/MIT

## Contribution

Contributions intentionally submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
