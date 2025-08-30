# Changelog

## 0.3.0

Breaking changes:

- Unified nightly/stable: signature now uses const template for locality.
- Function signatures changed from:
  - prefetch_read_data(addr: \*const u8, locality: i32)
  - prefetch_write_data(addr: \*const u8, locality: i32)
    to:
  - unsafe fn prefetch_read_data<T, const LOCALITY: i32>(addr: \*const T)
  - unsafe fn prefetch_write_data<T, const LOCALITY: i32>(addr: \*const T)
- Pointer now generic (*const T) instead of raw *const u8 (no manual casting needed).
- Locality passed as const generic (compile-time) instead of runtime parameter.
- Prefetch functions are now safe (no unsafe qualifier).

Example (old):
prefetch_read_data(ptr as \*const u8, 0);

Example (new):
unsafe { prefetch_read_data::<MyType, 0>(ptr); }
