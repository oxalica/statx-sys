extern crate statx_sys;
use libc::c_char;
use statx_sys::*;

#[test]
fn test_statx() {
    use libc::c_int;
    use std::{io, mem};

    const AT_FDCWD: c_int = -100; // Not contained in low version of libc

    // Working dir
    let c_path = b".\0";
    let c_path = c_path as *const _ as *mut c_char;

    let mut buf = unsafe { mem::zeroed() };
    let ret = unsafe { statx(AT_FDCWD, c_path, 0, STATX_ALL, &mut buf) };
    if ret != 0 {
        let err = io::Error::last_os_error();
        panic!("statx() failed: {:?}", err);
    } else {
        println!("statx() success: {:?}", buf);
    }
}
