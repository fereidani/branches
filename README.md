# Branches

[![Crates.io][crates-badge]][crates-url]
[![Documentation][doc-badge]][doc-url]
[![MIT licensed][mit-badge]][mit-url]

[crates-badge]: https://img.shields.io/crates/v/branches.svg?style=for-the-badge
[crates-url]: https://crates.io/crates/branches
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge
[mit-url]: https://github.com/fereidani/branches/blob/master/LICENSE
[doc-badge]: https://img.shields.io/docsrs/branches?style=for-the-badge
[doc-url]: https://docs.rs/branches

`branches` provides branch hinting and control functions for Rust, using built-in Rust features on stable and `core::intrinsics` on nightly.

## Usage

To use `branches`, add the following to your `Cargo.toml` file:

```toml
[dependencies]
branches = "0.1.0"
```

Then, import the `branches` crate and use the branch detection functions like so:

```rust
use branches::likely;

fn is_even(num: i32) -> bool {
    likely(num % 2 == 0)
}
```

## Functions

The following functions are provided by `branches`:

- `likely(b: bool) -> bool`: Returns back input value but leaves hints for the compiler that the statement is likely to be true.
- `unlikely(b: bool) -> bool`: Returns back input value but leaves hints for the compiler that the statement is unlikely to be true.
- `assume(b: bool)`: Assumes that the `b` is always true, and will cause undefined behavior if it is not. On stable Rust, this function uses `core::hint::unreachable_unchecked()` to achieve the same effect.
- `abort()`: Aborts the execution of the process immediately and without any cleanup.

## License

`branches` is licensed under the MIT license. See the `LICENSE` file for more information.
