# Changelog

## 0.4.5

Fixes (see MIGRATE.md for details and migration notes):

- Prefetch `LOCALITY` was interpreted with inverted semantics on nightly (`0` meant non-temporal instead of L1). Nightly now translates to the intrinsics' convention so `0 = L1, 1 = L2, 2 = L3, other = non-temporal` holds on every toolchain, as documented.
- Prefetch functions failed to compile on 32-bit ARM (AArch64-only `prfm`) and riscv64 (x86 mnemonics). 32-bit ARM is now a no-op; riscv64 emits the correct `prefetch.r`/`prefetch.w` (Zicbop) instructions when built with `-C target-feature=+zicbop` and is a no-op otherwise.
- A `LOCALITY` outside `0..=3` crashed the compiler on nightly; out-of-range values are now clamped to non-temporal.
- `abort()` with the `std` feature now always calls `std::process::abort()` (nightly previously executed a trap instruction, raising `SIGILL` instead of `SIGABRT`). Without `std` on stable it now panics with a clear `branches::abort() called` message instead of `unreachable!()`.
- On 32-bit x86, write prefetch now emits `prefetcht0` instead of `prefetchw`, which faults on CPUs without the PRFCHW extension.
- On AArch64, prefetch now maps `LOCALITY` 1/2 to L2/L3 (`pstl2keep`/`pstl3keep`/`pldl2keep`/`pldl3keep`) and other values to streaming prefetches, matching the documented convention and nightly behavior.
- `mark_unlikely` lost its documentation and example on rustc >= 1.95; the docs are back.
- `likely`/`unlikely` are now `#[must_use]`.

## 0.3.0

Breaking changes:

- Unified nightly/stable: signature now uses const template for locality.
- Function signatures changed from:
  - prefetch_read_data(addr: \*const u8, locality: i32)
  - prefetch_write_data(addr: \*const u8, locality: i32)
    to:
  - fn prefetch_read_data<T, const LOCALITY: i32>(addr: \*const T)
  - fn prefetch_write_data<T, const LOCALITY: i32>(addr: \*const T)
- Pointer now generic (*const T) instead of raw *const u8 (no manual casting needed).
- Locality passed as const generic (compile-time) instead of runtime parameter.
- Prefetch functions are now safe (no unsafe qualifier).

Example (old):
prefetch_read_data(ptr as \*const u8, 0);

Example (new):
prefetch_read_data::<MyType, 0>(ptr);
