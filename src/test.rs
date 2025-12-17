#[cfg(test)]
mod tests {
    #[cfg(feature = "prefetch")]
    use crate::{prefetch_read_data, prefetch_write_data};

    use crate::{abort, assume, likely, unlikely};
    use core::sync::atomic::{AtomicUsize, Ordering};

    // If this file is an integration test (tests/tests.rs), use the crate name.
    // Adjust the crate name below if different.

    // Try both styles to be resilient whether this is an integration or internal test module.
    #[allow(unused_imports)]
    #[test]
    fn test_likely_identity() {
        for &b in &[true, false] {
            assert_eq!(likely(b), b);
        }
    }

    #[test]
    fn test_unlikely_identity() {
        for &b in &[true, false] {
            assert_eq!(unlikely(b), b);
        }
    }

    #[test]
    fn test_likely_unlikely_mixed() {
        let data = [true, false, true, true, false];
        let mut c1 = 0usize;
        let mut c2 = 0usize;
        for &b in &data {
            if likely(b) {
                c1 += 1;
            }
            if unlikely(!b) {
                c2 += 1;
            }
        }
        assert_eq!(c1, 3);
        assert_eq!(c2, 2);
    }

    #[test]
    fn test_assume_true_no_effect() {
        // Use a computation the optimizer cannot trivially fold away.
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        for i in 0..100 {
            unsafe {
                assume(i < 100);
            }
            COUNTER.fetch_add(1, Ordering::Relaxed);
        }
        assert_eq!(COUNTER.load(Ordering::Relaxed), 100);
    }

    #[test]
    fn test_assume_in_loop_logic_preserved() {
        let mut sum = 0usize;
        for i in 1..=32 {
            unsafe {
                assume(i <= 32);
            }
            sum += i;
        }
        assert_eq!(sum, (32 * 33) / 2);
    }

    #[test]
    #[cfg(feature = "prefetch")]
    fn test_prefetch_read_variants() {
        let buf = [0u8; 64];
        let ptr = buf.as_ptr();
        // Just ensure it is safe w.r.t. memory (does not mutate / crash).
        prefetch_read_data::<_, 0>(ptr);
        prefetch_read_data::<_, 1>(ptr);
        prefetch_read_data::<_, 2>(ptr);
        prefetch_read_data::<_, 3>(ptr);

        assert_eq!(buf.iter().sum::<u8>(), 0);
    }

    #[test]
    #[cfg(feature = "prefetch")]
    fn test_prefetch_write_variants() {
        let buf = [1u8; 32];
        let ptr = buf.as_ptr();
        prefetch_write_data::<_, 0>(ptr);
        prefetch_write_data::<_, 1>(ptr);
        prefetch_write_data::<_, 2>(ptr);
        prefetch_write_data::<_, 3>(ptr);
        assert_eq!(buf.iter().sum::<u8>(), 32); // Ensure unchanged.
    }

    #[test]
    #[cfg(feature = "prefetch")]
    fn test_prefetch_multiple_addresses() {
        let mut big = [0u8; 256];
        for (i, b) in big.iter_mut().enumerate() {
            *b = (i & 0xFF) as u8;
        }
        for chunk in big.chunks(16) {
            prefetch_read_data::<_, 0>(chunk.as_ptr());
            prefetch_write_data::<_, 1>(chunk.as_ptr());
        }

        let checksum: u32 = big.iter().map(|&b| b as u32).sum();
        assert_eq!(checksum, (0u32..256).sum());
    }

    // Always check the symbol has the expected extern "C" signature without calling it.
    #[test]
    fn test_abort_signature() {
        let _f: extern "C" fn() -> ! = abort;
        let _ = _f as *const ();
    }

    // Smoke test combining all utilities to ensure they can coexist.
    #[test]
    #[cfg(feature = "prefetch")]
    fn test_prefetch_combined_usage_smoke() {
        let arr = [10u8, 20, 30, 40];
        let mut acc = 0u32;
        for &v in &arr {
            if likely(v % 20 != 0) {
                acc += v as u32;
            } else if unlikely(v == 20) {
                acc += 1;
            }
            unsafe {
                assume(v <= 40);
            }
            prefetch_read_data::<_, 0>(&arr);
        }
        // v % 20 != 0 for 10 and 30, so 10 + 30; 20 triggers unlikely branch (+1); 40 hits first branch (40%20==0 false) and second (40==20 false) so not added.
        assert_eq!(acc, 41);
    }
}
