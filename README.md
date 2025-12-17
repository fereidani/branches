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

To use `branches`, use following command:

```bash
cargo add branches
```

For a no_std environment, disable the default features(std and prefetch) by using following command:

```bash
cargo add branches --no-default-features
```

For a no_std with prefetch feature:

```bash
cargo add branches --no-default-features --features prefetch
```

## Functions

The following functions are provided by `branches`:

- `likely(b: bool) -> bool`: Returns the input value but provides hints for the compiler that the statement is likely to be true.
- `unlikely(b: bool) -> bool`: Returns the input value but provides hints for the compiler that the statement is unlikely to be true.
- `assume(b: bool)`: Assumes that the input condition is always true and causes undefined behavior if it is not. On stable Rust, this function uses `core::hint::unreachable_unchecked()` to achieve the same effect.
- `abort()`: Aborts the execution of the process immediately and without any cleanup.
- `prefetch_read_data<T, const LOCALITY: i32>(addr: *const T)`: Hints the CPU to load data at `addr` into cache for an upcoming read. `LOCALITY` selects cache behavior (e.g. 0 = L1, 1 = L2, 2 = L3, other = non‑temporal or arch default).
- `prefetch_write_data<T, const LOCALITY: i32>(addr: *const T)`: Hints the CPU to load a line for an upcoming write. Same `LOCALITY` semantics as above.

Guidelines:

- Only prefetch a small distance ahead (tune empirically).
- Too-far or excessive prefetching can evict useful cache lines.
- Never rely on prefetch for correctness; it is purely a performance hint.

### Likely/Unlikely example

Here's an example of how the `likely` can be used to optimize a function:

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

To understand the specific effect of likely and unlikely, consider the following example:

```rust
use branches::likely;

#[inline(never)]
pub fn tracker(v: usize) {
    core::hint::black_box(v);
}

#[inline(never)]
pub fn example(unknown: bool) {
    if likely(unknown){
        tracker(123)
    }else{
        tracker(255)
    }
}
```

This produces the following x86-64 assembly:

```assembly
example::example::h8ce045666cbb1dd5:
        mov     eax, edi
        mov     edi, 123
        test    eax, eax
        je      .LBB0_1
        jmp     qword ptr [rip + example::tracker::h1c31dda456fa4d53@GOTPCREL]
.LBB0_1:
        mov     edi, 255
        jmp     qword ptr [rip + example::tracker::h1c31dda456fa4d53@GOTPCREL]
```

Now, if we replace `likely(unknown)` with `unlikely(unknown)`:

```assembly
example::example::h8ce045666cbb1dd5:
        test    edi, edi
        jne     .LBB0_1
        mov     edi, 255
        jmp     qword ptr [rip + example::tracker::h1c31dda456fa4d53@GOTPCREL]
.LBB0_1:
        mov     edi, 123
        jmp     qword ptr [rip + example::tracker::h1c31dda456fa4d53@GOTPCREL]
```

As shown by the swapped positions of the code handling 123 and 255, the compiler eliminates the unconditional jump in the likely path (or places the jump in the unlikely path). This straight-line execution in the expected branch allows the likely path to run faster.

### Prefetch example

Loop manual prefetch example:

```rust
#[cfg(feature="prefetch")]
use branches::{prefetch_read_data, prefetch_write_data};
#[cfg(feature="prefetch")]
pub fn accumulate(a: &[u64], out: &mut [u64]) -> u64 {
    prefetch_read_data::<_, 0>(&a);
    prefetch_write_data::<_, 0>(&out);
    let mut sum = 0u64;
    let len = a.len().min(out.len());
    // Process in cache‑line sized blocks (assume 128‑byte cache line)
    const CACHE_LINE_BYTES: usize = 128;
    const ELEMS_PER_LINE: usize = CACHE_LINE_BYTES / core::mem::size_of::<u64>();

    let mut i = 0;
    while i < len {
        // Prefetch next cache line (read + future write)
        let next = i + ELEMS_PER_LINE;
        // There is no bug here, it is safe to prefetch memory out of bound!
        // Having `if < len` here reduces your performance.
        prefetch_read_data::<_, 0>(a.as_ptr().wrapping_add(next));
        prefetch_write_data::<_, 0>(a.as_ptr().wrapping_add(next));
        // Inner loop over one cache line
        let end = next.min(len);
        // The compiler can (partially) unroll this inner loop because (end - i)
        // is bounded by ELEMS_PER_LINE. For the final, shorter chunk (< ELEMS_PER_LINE)
        // it emits the scalar fallback.
        for j in i..end {
            sum += a[j];
            out[j] = sum;
        }
        i = end;
    }
    sum
}
```

By correctly using the functions provided by branches, you can achieve a 10-20% improvement in the performance of your algorithms.

## License

`branches` is licensed under the MIT license. See the `LICENSE` file for more information.
