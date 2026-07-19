#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(branches_nightly, feature(core_intrinsics))]
#![cfg_attr(branches_nightly, allow(internal_features))]
/// Provides branch detection functions for Rust, using built-in Rust features
/// on stable and core::intrinsics on nightly.

// No one likes to visit this function.
//
// It must stay an out-of-line call: the whole trick relies on LLVM seeing a
// call to a `#[cold]` function inside the branch. Any form of inlining
// (`#[inline]` or `#[inline(always)]`) removes the call during optimization
// and with it the hint, turning `likely`/`unlikely` into no-ops.
#[cfg(all(branches_stable, not(rustc_ge_1_95_0)))]
#[inline(never)]
#[cold]
const fn cold_and_empty() {}

#[cfg(all(branches_stable, rustc_ge_1_95_0))]
use core::hint::cold_path as cold_and_empty;

/// Aborts the execution of the process immediately and without any cleanup.
///
/// This function is used to indicate a critical and unrecoverable error in the program.
/// It terminates the process immediately without performing any cleanup or running destructors.
///
/// This function is safe to call, so it does not require an unsafe block.
/// Therefore, implementations must not require the user to uphold any safety invariants.
///
/// If the std feature is enabled, this function calls std::process::abort()
/// which is a more user-friendly and stable way of aborting the process.
///
/// If the std feature is disabled, this function panics by calling panic!().
/// In this case, by using `extern "C"` this function is guaranteed to not unwind.
#[cold]
pub extern "C" fn abort() -> ! {
    #[cfg(branches_stable)]
    {
        #[cfg(not(feature = "std"))]
        unreachable!();
        #[cfg(feature = "std")]
        std::process::abort();
    }
    #[cfg(branches_nightly)]
    core::intrinsics::abort()
}

/// Informs the optimizer that a condition is always true.
///
/// If the condition is actually false, the behavior is undefined.
///
/// This intrinsic doesn't generate any code. Instead, it tells the optimizer
/// to preserve the condition for optimization passes. This can interfere with
/// optimization of surrounding code and reduce performance, so avoid using it
/// if the optimizer can already discover the invariant on its own or if it
/// doesn't enable any significant optimizations.
///
/// # Safety
///
/// This intrinsic is marked unsafe because it can result in undefined behavior
/// if the condition passed to it is false.
#[inline(always)]
pub unsafe fn assume(b: bool) {
    #[cfg(branches_stable)]
    {
        // Rust >= 1.81.0: use the newer `assert_unchecked` hint.
        #[cfg(rustc_ge_1_81_0)]
        {
            core::hint::assert_unchecked(b)
        }
        // Rust < 1.81.0: fall back to the older `unreachable_unchecked`.
        #[cfg(not(rustc_ge_1_81_0))]
        {
            if !b {
                core::hint::unreachable_unchecked()
            }
        }
    }
    #[cfg(branches_nightly)]
    core::intrinsics::assume(b)
}

/// Hints to the compiler that the branch condition is likely to be true.
/// Returns the value passed to it.
///
/// This intrinsic is primarily used with `if` statements.
/// Using it in other contexts may not have any effect.
///
/// Unlike most intrinsics, this function is safe to call and doesn't require an `unsafe` block.
/// Therefore, implementations must not require the user to uphold any safety invariants.
#[inline(always)]
pub fn likely(b: bool) -> bool {
    #[cfg(branches_stable)]
    {
        if !b {
            cold_and_empty();
        }
        b
    }
    #[cfg(branches_nightly)]
    core::intrinsics::likely(b)
}

/// Marks a code block as cold, indicating to the compiler that it is unlikely to be called.
/// This can help the compiler optimize for the common case.
///
/// This function does not take any arguments and does not return any value.
/// It is primarily used to mark functions or code paths that are rarely executed,
/// such as error handling or panic paths.

/// Example: marking the `None` variant of an `Option` as unlikely.
///
/// In many hot paths an `Option<T>` is expected to be `Some`.  
/// By marking the `None` arm using `mark_unlikely` we give the optimizer a hint
/// that this branch is rarely taken.
///
/// ```rust
/// use branches::{mark_unlikely};
///
/// #[derive(Debug)]
/// enum Status {
///     Ok(i32),
///     Err(String),
/// }
///
/// fn get_value(status: Status) -> i32 {
///     match status {
///         Status::Ok(v) => v,
///         // The error case is rare, hint the compiler accordingly.
///         Status::Err(err) => {
///             mark_unlikely();
///             eprintln!("unexpected error: {:?}", err);
///             -1
///         }
///     }
/// }
/// ```
#[cfg(not(rustc_ge_1_95_0))]
#[cold]
#[inline(never)]
pub const fn mark_unlikely() {}
#[cfg(rustc_ge_1_95_0)]
pub use core::hint::cold_path as mark_unlikely;

/// Hints to the compiler that the branch condition is unlikely to be true.
/// Returns the value passed to it.
///
/// This intrinsic is primarily used with `if` statements.
/// Using it in other contexts may not have any effect.
///
/// Unlike most intrinsics, this function is safe to call and doesn't require an `unsafe` block.
/// Therefore, implementations must not require the user to uphold any safety invariants.
#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    #[cfg(branches_stable)]
    {
        if b {
            cold_and_empty();
        }
        b
    }
    #[cfg(branches_nightly)]
    core::intrinsics::unlikely(b)
}

/// Prefetches data for reading into the cache.
///
/// This function hints to the CPU that the data at the given address
/// will be read soon, allowing the CPU to load the data into the cache
/// in advance. This can improve performance by reducing cache misses.
///
/// Prefetching is only a hint and never affects the observable behavior of
/// the program: it is safe to call with any pointer, including dangling or
/// out-of-bounds pointers.
///
/// # Arguments
///
/// * `addr` - A pointer to the data to prefetch.
/// * `LOCALITY` - The cache level to prefetch into: `0` = L1, `1` = L2,
///   `2` = L3, any other value = non-temporal. The convention is identical
///   on stable and nightly toolchains.
///
/// # Supported architectures
///
/// On stable, the hint is emitted on `x86`/`x86_64` (with the `sse` target
/// feature, enabled by default on `x86_64` and `i686` targets), `aarch64`,
/// and `riscv64` when compiled with the `zicbop` target feature
/// (`-C target-feature=+zicbop`). On other targets this compiles to a
/// no-op. On nightly, the hint is lowered by LLVM for every architecture
/// that supports one.
#[inline(always)]
#[cfg(feature = "prefetch")]
pub fn prefetch_read_data<T, const LOCALITY: i32>(addr: *const T) {
    let _ = addr;
    #[cfg(branches_stable)]
    {
        #[cfg(all(
            any(target_arch = "x86", target_arch = "x86_64"),
            target_feature = "sse"
        ))]
        unsafe {
            match LOCALITY {
                0 => core::arch::asm!(
                    "prefetcht0 [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L1 cache
                1 => core::arch::asm!(
                    "prefetcht1 [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L2 cache
                2 => core::arch::asm!(
                    "prefetcht2 [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L3 cache
                _ => core::arch::asm!(
                    "prefetchnta [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // Non-temporal
            }
        }

        // `prfm` only exists on AArch64; 32-bit ARM would need `pld`, which
        // not every 32-bit ARM target supports, so arm stays a no-op.
        #[cfg(target_arch = "aarch64")]
        unsafe {
            match LOCALITY {
                0 => core::arch::asm!(
                    "prfm pldl1keep, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L1 cache
                1 => core::arch::asm!(
                    "prfm pldl2keep, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L2 cache
                2 => core::arch::asm!(
                    "prfm pldl3keep, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L3 cache
                _ => core::arch::asm!(
                    "prfm pldl1strm, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // Non-temporal (streaming)
            }
        }

        // The Zicbop extension is not part of the baseline riscv64gc target,
        // so the instruction is only emitted when the feature is enabled.
        #[cfg(all(target_arch = "riscv64", target_feature = "zicbop"))]
        unsafe {
            core::arch::asm!(
                "prefetch.r 0({})",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            );
        }

        // this requires unstable asm feature, uncomment when stabilized
        //#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
        //unsafe {
        //    core::arch::asm!(
        //        "dcbt 0, {}",
        //        in(reg) addr,
        //        options(nostack, readonly, preserves_flags)
        //    );
        //}
    }
    #[cfg(branches_nightly)]
    {
        // `core::intrinsics` uses the opposite locality convention
        // (0 = no locality .. 3 = maximally local), so translate to keep
        // stable and nightly behavior identical. The catch-all arm also
        // keeps out-of-range values from reaching LLVM, which only accepts
        // 0..=3 and crashes otherwise.
        match LOCALITY {
            0 => core::intrinsics::prefetch_read_data::<_, 3>(addr),
            1 => core::intrinsics::prefetch_read_data::<_, 2>(addr),
            2 => core::intrinsics::prefetch_read_data::<_, 1>(addr),
            _ => core::intrinsics::prefetch_read_data::<_, 0>(addr),
        }
    }
}

/// Prefetches data for writing into the cache.
///
/// This function hints to the CPU that the data at the given address
/// will be written soon, allowing the CPU to load the data into the cache
/// in advance. This can improve performance by reducing cache misses.
///
/// Prefetching is only a hint and never affects the observable behavior of
/// the program: it is safe to call with any pointer, including dangling or
/// out-of-bounds pointers.
///
/// # Arguments
///
/// * `addr` - A pointer to the data to prefetch.
/// * `LOCALITY` - The cache level to prefetch into: `0` = L1, `1` = L2,
///   `2` = L3, any other value = non-temporal. The convention is identical
///   on stable and nightly toolchains. On `x86_64` there is a single
///   write-prefetch instruction, so `LOCALITY` is ignored there.
///
/// # Supported architectures
///
/// On stable, the hint is emitted on `x86`/`x86_64` (with the `sse` target
/// feature, enabled by default on `x86_64` and `i686` targets), `aarch64`,
/// and `riscv64` when compiled with the `zicbop` target feature
/// (`-C target-feature=+zicbop`). On other targets this compiles to a
/// no-op. On nightly, the hint is lowered by LLVM for every architecture
/// that supports one.
#[inline(always)]
#[cfg(feature = "prefetch")]
pub fn prefetch_write_data<T, const LOCALITY: i32>(addr: *const T) {
    let _ = addr;
    #[cfg(branches_stable)]
    {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            core::arch::asm!(
                "prefetchw [{}]",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            ) // Write-prefetch for L1/L2/L3 cache
        }

        // 32-bit x86: `prefetchw` faults on CPUs without the PRFCHW/3DNow!
        // extension, so fall back to a plain read prefetch into L1, the same
        // strategy GCC and Clang use for `__builtin_prefetch(p, 1)` there.
        #[cfg(all(target_arch = "x86", target_feature = "sse"))]
        unsafe {
            core::arch::asm!(
                "prefetcht0 [{}]",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            )
        }

        // `prfm` only exists on AArch64; 32-bit ARM would need `pldw`, which
        // requires the MP extension, so arm stays a no-op.
        #[cfg(target_arch = "aarch64")]
        unsafe {
            match LOCALITY {
                0 => core::arch::asm!(
                    "prfm pstl1keep, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L1 cache
                1 => core::arch::asm!(
                    "prfm pstl2keep, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L2 cache
                2 => core::arch::asm!(
                    "prfm pstl3keep, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L3 cache
                _ => core::arch::asm!(
                    "prfm pstl1strm, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // Non-temporal (streaming)
            }
        }

        // The Zicbop extension is not part of the baseline riscv64gc target,
        // so the instruction is only emitted when the feature is enabled.
        #[cfg(all(target_arch = "riscv64", target_feature = "zicbop"))]
        unsafe {
            core::arch::asm!(
                "prefetch.w 0({})",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            );
        }

        // this requires unstable asm feature, uncomment when stabilized
        //#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
        //unsafe {
        //    core::arch::asm!(
        //        "dcbtst 0, {}",
        //        in(reg) addr,
        //        options(nostack, readonly, preserves_flags)
        //    ); // Write-prefetch
        // }
    }
    #[cfg(branches_nightly)]
    {
        // `core::intrinsics` uses the opposite locality convention
        // (0 = no locality .. 3 = maximally local), so translate to keep
        // stable and nightly behavior identical. The catch-all arm also
        // keeps out-of-range values from reaching LLVM, which only accepts
        // 0..=3 and crashes otherwise.
        match LOCALITY {
            0 => core::intrinsics::prefetch_write_data::<_, 3>(addr),
            1 => core::intrinsics::prefetch_write_data::<_, 2>(addr),
            2 => core::intrinsics::prefetch_write_data::<_, 1>(addr),
            _ => core::intrinsics::prefetch_write_data::<_, 0>(addr),
        }
    }
}
