#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs, missing_debug_implementations)]
#![cfg_attr(branches_nightly, feature(core_intrinsics))]
#![cfg_attr(branches_nightly, allow(internal_features))]
/// Provides branch detection functions for Rust, using built-in Rust features
/// on stable and core::intrinsics on nightly.

// No one likes to visit this function.
#[cfg(branches_stable)]
#[inline(always)]
#[cold]
const fn cold_and_empty() {}

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
#[cold]
#[inline(always)]
#[cfg(branches_stable)]
pub const fn mark_unlikely() {}
#[cfg(branches_nightly)]
pub use core::intrinsics::cold_path as mark_unlikely;

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
/// # Arguments
///
/// * `addr` - A pointer to the data to prefetch.
/// * `LOCALITY` - The cache locality to prefetch into.
#[inline(always)]
#[cfg(feature = "prefetch")]
pub fn prefetch_read_data<T, const LOCALITY: i32>(addr: *const T) {
    #[cfg(branches_stable)]
    {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
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

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
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
                _ => core::arch::asm!(
                    "prfm pldl3keep, [{}]",
                    in(reg) addr,
                    options(nostack, readonly, preserves_flags)
                ), // L3 or non-temporal
            }
        }

        #[cfg(target_arch = "riscv64")]
        unsafe {
            core::arch::asm!(
                "prefetch [{}]",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            );
        }

        #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
        unsafe {
            core::arch::asm!(
                "dcbt 0, {}",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            );
        }
    }
    #[cfg(branches_nightly)]
    core::intrinsics::prefetch_read_data::<_, LOCALITY>(addr)
}

/// Prefetches data for writing into the cache.
///
/// This function hints to the CPU that the data at the given address
/// will be written soon, allowing the CPU to load the data into the cache
/// in advance. This can improve performance by reducing cache misses.
///
/// # Arguments
///
/// * `addr` - A pointer to the data to prefetch.
/// * `LOCALITY` - The cache locality to prefetch into.
#[inline(always)]
#[cfg(feature = "prefetch")]
pub fn prefetch_write_data<T, const LOCALITY: i32>(addr: *const T) {
    #[cfg(branches_stable)]
    {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            core::arch::asm!(
                "prefetchw [{}]",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            ) // Write-prefetch for L1/L2/L3 cache
        }

        #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
        unsafe {
            core::arch::asm!(
                "prfm pstl1keep, [{}]",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            ); // Write-prefetch for L1 cache
        }

        #[cfg(target_arch = "riscv64")]
        unsafe {
            core::arch::asm!(
                "prefetchw [{}]",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            );
        }

        #[cfg(any(target_arch = "powerpc", target_arch = "powerpc64"))]
        unsafe {
            core::arch::asm!(
                "dcbtst 0, {}",
                in(reg) addr,
                options(nostack, readonly, preserves_flags)
            ); // Write-prefetch
        }
    }
    #[cfg(branches_nightly)]
    core::intrinsics::prefetch_write_data::<_, LOCALITY>(addr)
}

// tests
#[cfg(test)]
mod test;
