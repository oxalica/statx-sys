//! # Bindings to `statx` syscall.
//!
//! Note that `statx()` was added to Linux in kernel 4.11 .
//!
//! # See also
//! <http://man7.org/linux/man-pages/man2/statx.2.html>
#![no_std]
#![cfg(target_os = "linux")]
#![deny(warnings)]

use libc::syscall;
use libc::{c_char, c_int, c_uint};

/// Timestamp structure for the timestamps in struct statx.
///
/// tv_sec holds the number of seconds before (negative) or after (positive)
/// 00:00:00 1st January 1970 UTC.
///
/// tv_nsec holds a number of nanoseconds (0..999,999,999) after the tv_sec time.
///
/// __reserved is held in case we need a yet finer resolution.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct statx_timestamp {
    pub tv_sec: i64,
    pub tv_nsec: u32,
    __reserved: i32,
}

/// Structures for the extended file attribute retrieval system call
/// (statx()).
///
/// The caller passes a mask of what they're specifically interested in as a
/// parameter to statx().  What statx() actually got will be indicated in
/// st_mask upon return.
///
/// For each bit in the mask argument:
///
/// - if the datum is not supported:
///
///   - the bit will be cleared, and
///
///   - the datum will be set to an appropriate fabricated value if one is
///     available (eg. CIFS can take a default uid and gid), otherwise
///
///   - the field will be cleared;
///
/// - otherwise, if explicitly requested:
///
///   - the datum will be synchronised to the server if AT_STATX_FORCE_SYNC is
///     set or if the datum is considered out of date, and
///
///   - the field will be filled in and the bit will be set;
///
/// - otherwise, if not requested, but available in approximate form without any
///   effort, it will be filled in anyway, and the bit will be set upon return
///   (it might not be up to date, however, and no attempt will be made to
///   synchronise the internal state first);
///
/// - otherwise the field and the bit will be cleared before returning.
///
/// Items in STATX_BASIC_STATS may be marked unavailable on return, but they
/// will have values installed for compatibility purposes so that stat() and
/// co. can be emulated in userspace.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct statx {
    // 0x00
    /// What results were written \[uncond]
    pub stx_mask: u32,
    /// Preferred general I/O size \[uncond]
    pub stx_blksize: u32,
    /// Flags conveying information about the file \[uncond]
    pub stx_attributes: u64,

    // 0x10
    /// Number of hard links
    pub stx_nlink: u32,
    /// User ID of owner
    pub stx_uid: u32,
    /// Group ID of owner
    pub stx_gid: u32,
    /// File mode
    pub stx_mode: u16,
    __spare0: [u16; 1],

    // 0x20
    /// Inode number
    pub stx_ino: u64,
    /// File size
    pub stx_size: u64,
    /// Number of 512-byte blocks allocated
    pub stx_blocks: u64,
    /// Mask to show what's supported in stx_attributes
    pub stx_attributes_mask: u64,

    // 0x40
    pub stx_atime: statx_timestamp, /* Last access time */
    pub stx_btime: statx_timestamp, /* File creation time */
    pub stx_ctime: statx_timestamp, /* Last attribute change time */
    pub stx_mtime: statx_timestamp, /* Last data modification time */

    /* 0x80 */
    /// Device ID of special file \[if bdev/cdev]
    pub stx_rdev_major: u32,
    pub stx_rdev_minor: u32,
    /// ID of device containing file \[uncond]
    pub stx_dev_major: u32,
    pub stx_dev_minor: u32,

    // 0x90
    /// Spare space for future expansion
    __spare2: [u64; 14],
    // 0x100
}

mod syscall;
pub use syscall::SYS_statx;

// Flags
pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_NO_AUTOMOUNT: c_int = 0x800;
pub const AT_EMPTY_PATH: c_int = 0x1000;

pub const AT_STATX_SYNC_AS_STAT: c_int = 0x0000;
pub const AT_STATX_FORCE_SYNC: c_int = 0x2000;
pub const AT_STATX_SYNC_TYPE: c_int = 0x6000;
pub const AT_STATX_DONT_SYNC: c_int = 0x4000;

pub const STATX_TYPE: c_uint = 0x0000_0001;
pub const STATX_MODE: c_uint = 0x0000_0002;
pub const STATX_NLINK: c_uint = 0x0000_0004;
pub const STATX_UID: c_uint = 0x0000_0008;
pub const STATX_GID: c_uint = 0x0000_0010;
pub const STATX_ATIME: c_uint = 0x0000_0020;
pub const STATX_MTIME: c_uint = 0x0000_0040;
pub const STATX_CTIME: c_uint = 0x0000_0080;
pub const STATX_INO: c_uint = 0x0000_0100;
pub const STATX_SIZE: c_uint = 0x0000_0200;
pub const STATX_BLOCKS: c_uint = 0x0000_0400;
pub const STATX_BASIC_STATS: c_uint = 0x0000_07ff;
pub const STATX_BTIME: c_uint = 0x0000_0800;
pub const STATX_ALL: c_uint = 0x0000_0fff;
pub const STATX__RESERVED: c_uint = 0x8000_0000;

// File attributes.

pub const STATX_ATTR_COMPRESSED: c_int = 0x0000_0004;
pub const STATX_ATTR_IMMUTABLE: c_int = 0x0000_0010;
pub const STATX_ATTR_APPEND: c_int = 0x0000_0020;
pub const STATX_ATTR_NODUMP: c_int = 0x0000_0040;
pub const STATX_ATTR_ENCRYPTED: c_int = 0x0000_0800;

pub const STATX_ATTR_AUTOMOUNT: c_int = 0x0000_1000;

/// statx - get file status (extended)
///
/// See also:
/// <http://man7.org/linux/man-pages/man2/statx.2.html>
pub unsafe fn statx(
    dirfd: c_int,
    pathname: *const c_char,
    flags: c_int,
    mask: c_uint,
    statxbuf: *mut statx,
) -> c_int {
    syscall(SYS_statx, dirfd, pathname, flags, mask, statxbuf) as c_int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_struct_layout() {
        use core::mem::size_of;
        use memoffset::offset_of;

        assert_eq!(size_of::<statx>(), 0x100);
        assert_eq!(size_of::<statx_timestamp>(), 16);

        assert_eq!(offset_of!(statx, stx_mask), 0);
        assert_eq!(offset_of!(statx, stx_nlink), 0x10);
        assert_eq!(offset_of!(statx, stx_ino), 0x20);
        assert_eq!(offset_of!(statx, stx_atime), 0x40);
        assert_eq!(offset_of!(statx, stx_rdev_major), 0x80);
        assert_eq!(offset_of!(statx, __spare2), 0x90);
    }
}
