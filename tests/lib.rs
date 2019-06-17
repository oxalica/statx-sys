extern crate statx_sys;
use statx_sys::*;
use libc::c_char;

#[test]
#[ignore]
fn test_stat() {
    use libc::{__errno_location, strerror, AT_FDCWD};
    use std::cell::UnsafeCell;
    use std::mem::zeroed;
    use std::os::unix::ffi::OsStrExt;
    use std::path::Path;

    const PATH: &str = "."; // Working dir.

    let mut c_path_buf: Vec<u8> = Path::new(PATH).as_os_str().as_bytes().to_owned();
    c_path_buf.push(0); // '\0'
    let c_path = c_path_buf.as_ptr() as *mut c_char;

    let buf = UnsafeCell::new(unsafe { zeroed::<statx>() });
    let ret = unsafe { statx(AT_FDCWD, c_path, 0, STATX_ALL, buf.get()) };
    if ret != 0 {
        let errno = unsafe { *__errno_location() };
        let err_str: String = unsafe {
            let pstr = strerror(errno);
            assert!(!pstr.is_null());
            (0..)
                .map(|i| *pstr.offset(i) as u8 as char)
                .take_while(|c| *c != '\0')
                .collect()
        };
        panic!("statx() failed: {}", err_str);
    } else {
        println!("statx() success: {:?}", buf.into_inner());
    }
}
