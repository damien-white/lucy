<div align="center">
    <hr /><br/>
    <img src="assets/logo.svg" width="400" alt="Lucy logo" />
    <hr />
    <h3 align="center">  
         JSON parser service for simplifying your data pipelines
    </h3>

[![Crates.io](https://img.shields.io/crates/v/lucy.svg)](https://crates.io/crates/lucy)
[![Docs.rs](https://docs.rs/lucy/badge.svg)](https://docs.rs/lucy)
[![CI](https://github.com/dark-fusion/lucy/workflows/CI/badge.svg)](https://github.com/dark-fusion/lucy/actions)
[![Coverage Status](https://coveralls.io/repos/github/dark-fusion/lucy/badge.svg?branch=main)](https://coveralls.io/github/dark-fusion/lucy?branch=main)

</div>

## Description

Lucy is a fast and efficient JSON parsing service for building data pipelines.

Lucy parses JSON from sources that produce raw bytes, such as network sockets. This allows Lucy to
easily integrate with new and existing systems.

### JSON Syntax

The specification that defines valid JSON syntax and the format itself can be found
within [IETF RFC 8259](https://datatracker.ietf.org/doc/html/rfc8259).

### Project Goals

- Full support for JSON data types
    - `array`, `boolean`, `null`, `number`, `object`, `string`
- Expose an intuitive API that integrates easily with new and existing projects
- General optimization of internal functions for speed and resource-efficiency
    - Zero-copy operations
    - Limit number of allocations
    - Usage of `#[inline]` for hot paths and cold paths
- Limit the usage of third-party crates
    - External crates should only be pulled in if deemed absolutely necessary

**Please note**: Optimizations will be made wherever possible but will be "best-effort" until an MVP
is released.

## Technologies

### Languages and Tools

Lucy is written in [Rust](https://rust-lang.org/) for its high-performance at runtime, resource
efficiency and memory-safety.

Rust's vibrant ecosystem contains support for asynchronous IO and parallel processing (concurrency
and parallelism).

### External Dependencies

Lucy depends on a few well-maintained crates:

- [nom](https://github.com/Geal/nom)
- TBA

## License

Licensed under the [MIT License](LICENSE):

- You may also find a copy at http://opensource.org/licenses/MIT

## Contribution

Contributions intentionally submitted for inclusion in the work by you shall be licensed as above,
without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
