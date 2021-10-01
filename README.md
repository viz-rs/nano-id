<h1 align="center">nano-id</h1>

<div align="center">
  <p>
    <strong>A tiny, secure, URL-friendly, unique string ID generator for Rust</strong>
  </p>
  <p>Inspired by <a href="https://github.com/ai/nanoid">nanoid</a></p>
</div>

<div align="center">
  <!-- Docs.rs docs -->
  <a href="https://docs.rs/nano-id">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="Docs.rs docs" /></a>
  <!-- Crates version -->
  <a href="https://crates.io/crates/nano-id">
    <img src="https://img.shields.io/crates/v/nano-id.svg?style=flat-square"
    alt="Crates.io version" /></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/nano-id">
    <img src="https://img.shields.io/crates/d/nano-id.svg?style=flat-square"
      alt="Download" /></a>
</div>

## Features

* **Base58**: `ModueSymbhaswnPr123456789ABCDEFGHNRVfgctiUvzKqYTJkLxpZXjQW`, (`123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz`)
* **Base62**: `ModuleSymbhasOwnPr0123456789ABCDEFGHNRVfgctiUvzKqYTJkLxpZXIjQW`, (`A-Za-z0-9`)
* **Base64**: `ModuleSymbhasOwnPr-0123456789ABCDEFGHNRVfgctiUvz_KqYTJkLxpZXIjQW`, (`A-Za-z0-9_-`)
* **Random**: data by [getrandom][]
* **Alphabet**: custom with `gen!(uid, 64, b"_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ")`

## Installation

```shell
cargo add nano-id
```

## Usage

```rust
nano_id::base64::<21>();

// Custom ALPHABET
nano_id::gen!(
    uid,
    64,
    b"_-0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
);

uid::<21>();
```

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

[getrandom]: https://github.com/rust-random/getrandom
