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

`branches` provides branch hinting prediction and control functions for optimization of algorithms, using built-in Rust features on stable and `core::intrinsics` on nightly.

## Usage

To use `branches`, add the following to your `Cargo.toml` file:

```toml
[dependencies]
branches = "0.1"
```

## Functions

The following functions are provided by `branches`:

- `likely(b: bool) -> bool`: Returns the input value but provides hints for the compiler that the statement is likely to be true.
- `unlikely(b: bool) -> bool`: Returns the input value but provides hints for the compiler that the statement is unlikely to be true.
- `assume(b: bool)`:  Assumes that the input condition is always true and causes undefined behavior if it is not. On stable Rust, this function uses `core::hint::unreachable_unchecked()` to achieve the same effect.
- `abort()`: Aborts the execution of the process immediately and without any cleanup.

Here's an example of how you can use `likely` to optimize a function:

```rust
use branches::likely;

pub fn factorial(n: usize) -> usize {
    if likely(n > 1) {
        n * factorial(n - 1)
    } else {
        1
    }
}
```

By correctly using the functions provided by branches, you can achieve a 10-20% improvement in the performance of your algorithms.

## License

`branches` is licensed under the MIT license. See the `LICENSE` file for more information.
