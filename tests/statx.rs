extern crate statx_sys;
use libc::{c_char, ENOSYS};
use statx_sys::*;
use std::{io, mem};

fn run_statx(allow_nosys: bool) {
    // Test on root
    let c_path = b"/\0";
    let c_path = c_path as *const _ as *mut c_char;

    let mut buf = unsafe { mem::zeroed() };
    let ret = unsafe { statx(AT_FDCWD, c_path, 0, STATX_ALL, &mut buf) };
    if ret == 0 {
        println!("statx() success: {:?}", buf);
    } else {
        let err = io::Error::last_os_error();
        if allow_nosys && err.raw_os_error().unwrap() == ENOSYS {
            eprintln!("warning: statx() is not supported");
        } else {
            panic!("statx() failed: {:?}", err);
        }
    }
}

#[test]
fn test_stat_invoke() {
    run_statx(true);
}

/// Syscall `statx` has version requirement of Linux kernel,
/// so it is not always available.
#[test]
#[ignore]
fn test_stat_success() {
    run_statx(false);
}
