#![doc = include_str!("../README.md")]
#![warn(missing_docs, missing_debug_implementations)]

/// Provides branch detection functions for Rust, using built-in Rust features
/// on stable and core::intrinsics on nightly.
#[cfg(feature = "nightly")]
pub use core::intrinsics::{abort, assume, likely, unlikely};

// No one like to visit this function
#[cfg(not(feature = "nightly"))]
#[inline(always)]
#[cold]
fn cold_and_empty() {}

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
#[cfg(not(feature = "nightly"))]
#[cold]
pub extern "C" fn abort() -> ! {
    #[cfg(not(feature = "std"))]
    #[inline(always)]
    fn _abort() -> ! {
        panic!()
    }
    #[cfg(feature = "std")]
    #[inline(always)]
    fn _abort() -> ! {
        std::process::abort()
    }
    _abort()
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
#[cfg(not(feature = "nightly"))]
#[inline(always)]
pub unsafe fn assume(b: bool) {
    if !b {
        core::hint::unreachable_unchecked();
    }
}

/// Hints to the compiler that the branch condition is likely to be true.
/// Returns the value passed to it.
///
/// This intrinsic is primarily used with `if` statements.
/// Using it in other contexts may not have any effect.
///
/// Unlike most intrinsics, this function is safe to call and doesn't require an `unsafe` block.
/// Therefore, implementations must not require the user to uphold any safety invariants.
#[cfg(not(feature = "nightly"))]
#[inline(always)]
pub fn likely(b: bool) -> bool {
    if !b {
        cold_and_empty();
    }
    b
}

/// Hints to the compiler that the branch condition is unlikely to be true.
/// Returns the value passed to it.
///
/// This intrinsic is primarily used with `if` statements.
/// Using it in other contexts may not have any effect.
///
/// Unlike most intrinsics, this function is safe to call and doesn't require an `unsafe` block.
/// Therefore, implementations must not require the user to uphold any safety invariants.
#[cfg(not(feature = "nightly"))]
#[inline(always)]
pub fn unlikely(b: bool) -> bool {
    if b {
        cold_and_empty();
    }
    b
}
