# statx-sys

**This project is archived. Please use `libc` directly, or `rustix` for
non-libc targets.**

Bindings to `statx` syscall which is available in Linux kernel 4.11 ,
implemented by `syscall` syscall.

Man page of `statx`: http://man7.org/linux/man-pages/man2/statx.2.html

`statx` related `fn` and `struct`s are first included in [`libc`][libc] 0.2.56 .
And this crate provide an alternative which is compatible to [`libc`][libc] 0.2.3 ,
which is the lowest version with support for `syscall`.
For system without `statx` support, the `fn statx` of this crate will safely
return `ENOSYS` instead of a compile-time link error.

[libc]: https://crates.io/crates/libc
