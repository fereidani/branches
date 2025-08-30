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

`branches` provides branch prediction hints, control flow assumptions, abort, and manual data prefetch (read & write) helpers for performance optimization, using stable Rust primitives where available and falling back to `core::intrinsics` on nightly.

## Usage

To use `branches`, add the following to your `Cargo.toml` file:

```toml
[dependencies]
branches = "0.2"
```

For a no_std environment, disable the default features by adding the following to your `Cargo.toml` instead:

```toml
[dependencies]
branches = { version = "0.2", default-features = false }
```

## Functions

The following functions are provided by `branches`:

- `likely(b: bool) -> bool`: Returns the input value but provides hints for the compiler that the statement is likely to be true.
- `unlikely(b: bool) -> bool`: Returns the input value but provides hints for the compiler that the statement is unlikely to be true.
- `assume(b: bool)`: Assumes that the input condition is always true and causes undefined behavior if it is not. On stable Rust, this function uses `core::hint::unreachable_unchecked()` to achieve the same effect.
- `abort()`: Aborts the execution of the process immediately and without any cleanup.
- `prefetch_read_data<T, const LOCALITY: i32>(addr: *const T)`: Hints the CPU to load data at `addr` into cache for an upcoming read. `LOCALITY` selects cache behavior (e.g. 0 = L1, 1 = L2, 2 = L3, other = non‑temporal or arch default). Unsafe: `addr` must be a valid, properly aligned pointer; may not alias freed or unmapped memory.
- `prefetch_write_data<T, const LOCALITY: i32>(addr: *const T)`: Hints the CPU to load a line for an upcoming write. Same `LOCALITY` semantics as above. Unsafe for the same reasons, and you must ensure future writes are plausible (avoid prefetching arbitrary / constant addresses).

Guidelines:

- Only prefetch a small distance ahead (tune empirically).
- Too-far or excessive prefetching can evict useful cache lines.
- Never rely on prefetch for correctness; it is purely a performance hint.

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

Loop manual prefetch example:

```rust
use branches::{prefetch_read_data, prefetch_write_data};

pub fn accumulate(a: &[u64], out: &mut [u64]) -> u64 {
    let mut sum = 0u64;
    let len = a.len().min(out.len());
    for i in 0..len {
        // Prefetch the next iteration's data (read) into L1
        if i + 16 < len {
            unsafe { prefetch_read_data::<u64, 0>(a.as_ptr().wrapping_add(i + 16)); }
            unsafe { prefetch_write_data::<u64, 0>(out.as_ptr().wrapping_add(i + 16)); }
        }
        sum += a[i];
        out[i] = sum;
    }
    sum
}
```

By correctly using the functions provided by branches, you can achieve a 10-20% improvement in the performance of your algorithms.

## License

`branches` is licensed under the MIT license. See the `LICENSE` file for more information.
