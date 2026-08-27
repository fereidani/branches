# Changelog

## 0.4.6

- The `rustc_version` build-dependency is gone: the build script now reads `rustc -vV` itself, leaving the crate with zero dependencies. Recent releases of the `semver` crate (which `rustc_version` pulls in) ship manifests that cargo older than 1.60 cannot parse, which would have made the 1.51 MSRV unusable in practice.
- Prefetch hints are now emitted on `s390x` (`pfd`) and `powerpc`/`powerpc64` (`dcbt`/`dcbtst`), including `powerpc64le`. Both are gated on the rustc release that stabilized inline assembly for the architecture (1.84 for `s390x`, 1.95 for PowerPC); older compilers keep building the crate with prefetch as a no-op. Neither architecture has cache-level selection in its prefetch instruction, so `LOCALITY` is ignored there.

## 0.4.5

Fixes (see MIGRATE.md for details and migration notes):

- `likely`, `unlikely`, and `mark_unlikely` had no effect on stable rustc older than 1.95 since 0.4.2: `#[inline(always)]` on the internal cold helpers let the optimizer erase the cold call the hint relies on. The helpers are `#[inline(never)]` again and the hints work on all supported stable compilers.
- Prefetch `LOCALITY` was interpreted with inverted semantics on nightly (`0` meant non-temporal instead of L1). Nightly now translates to the intrinsics' convention so `0 = L1, 1 = L2, 2 = L3, other = non-temporal` holds on every toolchain, as documented.
- Prefetch functions failed to compile on 32-bit ARM (AArch64-only `prfm`) and riscv64 (x86 mnemonics). 32-bit ARM is now a no-op; riscv64 emits the correct `prefetch.r`/`prefetch.w` (Zicbop) instructions when built with `-C target-feature=+zicbop` and is a no-op otherwise.
- A `LOCALITY` outside `0..=3` crashed the compiler on nightly; out-of-range values are now clamped to non-temporal.
- `abort()` with the `std` feature now always calls `std::process::abort()` (nightly previously executed a trap instruction, raising `SIGILL` instead of `SIGABRT`). Without `std` on stable it now panics with a clear `branches::abort() called` message instead of `unreachable!()`.
- On 32-bit x86, write prefetch now emits `prefetcht0` instead of `prefetchw`, which faults on CPUs without the PRFCHW extension.
- On AArch64, prefetch now maps `LOCALITY` 1/2 to L2/L3 (`pstl2keep`/`pstl3keep`/`pldl2keep`/`pldl3keep`) and other values to streaming prefetches, matching the documented convention and nightly behavior.
- `mark_unlikely` lost its documentation and example on rustc >= 1.95; the docs are back.
- `likely`/`unlikely` are now `#[must_use]`.
- Declared `rust-version = "1.59"`; build script falls back to the stable code path instead of failing cryptically when the compiler version cannot be detected.
- README example fixes: `accumulate` prefetched the address of the slice reference instead of the buffer, and write-prefetched the input instead of the output buffer.

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
