extern crate statx_sys;
use libc::c_char;
use statx_sys::*;

#[test]
fn test_statx() {
    use libc::{__errno_location, c_int, strerror};
    use std::ffi::CStr;
    use std::mem::MaybeUninit;

    const AT_FDCWD: c_int = -100; // Not contained in low version of libc

    // Working dir
    let c_path = b".\0";
    let c_path = c_path as *const _ as *mut c_char;

    let mut buf = MaybeUninit::zeroed();
    let ret = unsafe { statx(AT_FDCWD, c_path, 0, STATX_ALL, buf.as_mut_ptr()) };
    if ret != 0 {
        let errno = unsafe { *__errno_location() };
        let err_str = unsafe {
            let pstr = strerror(errno);
            assert!(!pstr.is_null());
            CStr::from_ptr(pstr).to_owned()
        };
        panic!("statx() failed: ({}) {}", errno, err_str.to_string_lossy());
    } else {
        let buf = unsafe { buf.assume_init() };
        println!("statx() success: {:?}", buf);
    }
}
